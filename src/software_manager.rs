use crate::{dep_manager::{DependencyItem, DependencyItemList}, network_module::download_unit, run_profile::profile_handler};
use base64::decode;
use lazy_static::lazy_static;
use serde::de;
use reqwest::Error;
use simple_logger::SimpleLogger;
use tokio::runtime::Runtime;
use std::{
    collections::{HashMap, HashSet, LinkedList}, fs::File, io::Write, ptr::null, result, sync::{Arc, Mutex, MutexGuard}
};
use serde_json::{Value, json};
// 安装的软件包
#[derive(Debug)]
pub struct Software {
    pub inner: Mutex<InnerSoftware>,
    // 
    pub archive: String,
}

#[derive(Debug, Clone)]
pub struct InnerSoftware {
    // 指针，指向实际安装位置
    ptr: String,
    // 引用计数。 包括new version和 last version
    reference_count: i32,
    //
    dependency: Arc<DependencyItem>,
}

impl InnerSoftware{
    pub fn reference_count(&self) -> i32 {
        return self.reference_count;
    }
    pub fn dependency(&self) -> Arc<DependencyItem> {
        return self.dependency.clone();
    }
    // 对软件包的引用计数做修改
    pub fn add(&mut self) {
        self.reference_count += 1;
    }
}

impl Software {
    pub fn new(path: String, dependency: Arc<DependencyItem>) -> Arc<Software> {
        return Arc::new(Software {
            inner: Mutex::new(InnerSoftware {
                ptr: path,
                reference_count: 0,
                dependency: dependency.clone(),
            }),
            archive : dependency.clone().archive.clone()
        });
    }

    pub fn inner(&self) -> Option<MutexGuard<InnerSoftware>>{
        self.inner.lock().ok()
    }
   
}

// 管理器作为单例
lazy_static! {
    static ref SOFTWARE_MANAGER: Arc<Mutex<SoftwareManager>> =
        Arc::new(Mutex::new(SoftwareManager::new()));
}
#[inline(always)]
#[allow(dead_code)]
pub(super) fn software_manager() -> &'static Arc<Mutex<SoftwareManager>> {
    &SOFTWARE_MANAGER
}

pub enum SoftwareManagerError {
    CircularDependency(LinkedList<Arc<DependencyItem>>),
    DependencyNotFound(Arc<DependencyItem>),
    DependencyAlreadyInstalled(Arc<DependencyItem>),
    DependencyNotInstalled(Arc<DependencyItem>),
    // 下载时遇到错误
    DownloadError(String),
    // 安装时遇到错误
    InstallDependencyError(String),
    // 解析依赖时遇到错误
    ParseDependencyError(String),
    // 读取本地文件错误
    ReadLocalFileError(String),
    // 删除依赖时遇到错误
    RemoveDependencyError(String),
    // software加锁失败
    SoftwareLockError(String),
}
// 有一个对软件包做管理的类，持有所有下载的软件包
// 垃圾回收，删除所有引用为0的软件包
pub struct SoftwareManager {
    softwares: Vec<Arc<Software>>,
    hashmap: HashMap<Arc<DependencyItem>, Arc<Software>>,
}
impl SoftwareManager {
    fn new() -> SoftwareManager {
        return SoftwareManager {
            softwares: Vec::new(),
            hashmap: HashMap::new(),
        };
    }
    fn hashmap(&self) -> &HashMap<Arc<DependencyItem>, Arc<Software>> {
        return &self.hashmap;
    }
    // 删除所有引用数为0的依赖包
    // todo : 目前是删除所有计数为0的，但是考虑删除后减少它的依赖包的计数，考虑迭代删除
    pub fn garbage_collection(&mut self) -> Result<(), SoftwareManagerError> {
        let mut result = Ok(());
        // 检查引用是否为0 ，为0则删除文件，再从数组移除
        let mut indexes_to_remove = Vec::new();
        // 使用 for 循环找到要删除的元素的索引
        for (index, soft) in self.softwares.iter().enumerate() {
            let inner_guard = soft.inner();
            if inner_guard.is_none() {
                result = Err(SoftwareManagerError::SoftwareLockError(soft.archive.clone()));
                break;
            }
            let guard = inner_guard.unwrap();
            if guard.reference_count() == 0 {
                // todo 删除文件，并且它所依赖的文件的的计数要减少
                // if {
                //     result = Err(SoftwareManagerError::RemoveDependencyError("".to_string()));
                //     break;
                // }
                // todo 一个文件被删除，它所依赖的文件可能成为垃圾，需要被回收
                indexes_to_remove.push(index);
            }
        }
        // 根据索引删除元素
        for index in indexes_to_remove.iter().rev() {
            self.softwares.remove(*index);
        }
        if !result.is_err() {
            log::info!("garbage collection success");
        }
        return result;
    }
    // 添加一个新软件
    pub fn insert(
        &mut self,
        software: Arc<Software>,
        dependency: Arc<DependencyItem>,
    ) -> Result<(), SoftwareManagerError> {
        if !self.hashmap.contains_key(&dependency) {
            self.softwares.push(software.clone());
            self.hashmap.insert(dependency, software.clone());
        } else {
            return Err(SoftwareManagerError::DependencyAlreadyInstalled(dependency));
        }
        return Ok(());
    }
    // 检查dependency_list是否存在环
    pub fn check(
        &self,
        dependency_list: DependencyItemList,
    ) -> Result<LinkedList<Arc<DependencyItem>>, SoftwareManagerError> {
        // 用于判断是否已经下载或遍历时已经加入下载队列
        let mut hashmap: HashMap<Arc<DependencyItem>, bool> = HashMap::new();
        for key in self.hashmap.keys() {
            hashmap.insert(key.clone(), true);
        }
        // 用于记录下载队列，按顺序，无需其它依赖的排前面
        let mut download_list: LinkedList<Arc<DependencyItem>> = LinkedList::new();
        let mut hashset: HashSet<Arc<DependencyItem>> = HashSet::new();
        let mut linkedlist: LinkedList<Arc<DependencyItem>> = LinkedList::new();
        for dependency in dependency_list.dependencies {
            // 检测出环形
            match Self::dfs(
                self,
                dependency,
                &mut hashset,
                &mut linkedlist,
                &mut hashmap,
                &mut download_list,
            ) {
                Ok(b) => match b {
                    true => {}
                    false => return Err(SoftwareManagerError::CircularDependency(download_list)),
                },
                Err(e) => return Err(e),
            }
        }
        return Ok(download_list);
    }

    // 深度优先搜索查找环
    fn dfs(
        &self,
        dependency: Arc<DependencyItem>,
        hashset: &mut HashSet<Arc<DependencyItem>>,
        linkedlist: &mut LinkedList<Arc<DependencyItem>>,
        hashmap: &mut HashMap<Arc<DependencyItem>, bool>,
        download_list: &mut LinkedList<Arc<DependencyItem>>,
    ) -> Result<bool, SoftwareManagerError> {
        //
        if hashmap.contains_key(&dependency) {
            return Ok(true);
        }
        if !hashset.insert(dependency.clone()) {
            // 出现环形，用download list 装环形
            download_list.clear();
            for dep_tmp in linkedlist.iter().rev() {
                //println!("{} {} -> ", dep_tmp.archive, dep_tmp.version);
                download_list.push_back(dep_tmp.clone());
                if dep_tmp.eq(&dependency)   {
                    break;
                }
            }
            return Ok(false);
        }
        linkedlist.push_back(dependency.clone());
        // 根据dep获得dep的依赖
        let config = match Self::get_dep(dependency.clone()) {
            Ok(dependency_list) => dependency_list,
            Err(err) => return Err(err),
        };
        
        for dependency in config.dependencies {
            match self.dfs(dependency, hashset, linkedlist, hashmap, download_list) {
                Ok(b) => match b {
                    true => (),
                    false => return Ok(false),
                },
                Err(err) => return Err(err),
            }
        }
        linkedlist.pop_back();
        hashset.remove(&dependency);
        hashmap.insert(dependency.clone(), true);
        download_list.push_front(dependency.clone());
        return Ok(true);
    }

    // 根据下载队列来安装
    pub fn install(
        &mut self,
        download_list: LinkedList<Arc<DependencyItem>>,
    ) -> Result<(), SoftwareManagerError> {
        // 将下载依赖交给下载器依次下载
        for dependency in download_list.iter() {
            let result = download_unit().download_software(dependency.clone());
            let software;
            match result {
                Ok(s) => software = s,
                Err(err) => return Err(err),
            }
            //println!("{:?}", software);
            // 加入新下载的依赖包
            self.insert(software, dependency.clone());
        }
        return Ok(());
    }
    // 更新引用计数
    pub fn update_reference(&mut self, list: DependencyItemList) -> Result<(), SoftwareManagerError>{
        for dependency in list.dependencies {
            if self.hashmap().contains_key(&dependency){
                let software = self.hashmap().get(&dependency).unwrap();
                let inner_guard = software.inner();
                // 加锁
                if inner_guard.is_some() {
                    let mut guard = inner_guard.unwrap();
                    guard.add();
                }else {
                    return Err(SoftwareManagerError::SoftwareLockError(software.archive.clone()));
                }
            }else {
                // 如果dependency不存在
                return Err(SoftwareManagerError::DependencyNotFound(dependency.clone()));
            }
            
        }
        return Ok(());
    }
    // 查找依赖包的依赖
    pub fn get_dep(dep: Arc<DependencyItem>) -> Result<DependencyItemList, SoftwareManagerError> {
        // 通过网络获取该文件的配置文件
        return download_unit().get_dependency_list(dep.clone());
    }
}


