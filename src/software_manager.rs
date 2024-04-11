use crate::{dep_manager::{Dependency, DependencyList}, run_profile::profile_handler};
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
    dependency: Arc<Dependency>,
}

impl InnerSoftware{
    pub fn reference_count(&self) -> i32 {
        return self.reference_count;
    }
    pub fn dependency(&self) -> Arc<Dependency> {
        return self.dependency.clone();
    }
    // 对软件包的引用计数做修改
    pub fn add(&mut self) {
        self.reference_count += 1;
    }
}

impl Software {
    pub fn new(path: String, dependency: Arc<Dependency>) -> Arc<Software> {
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
    CircularDependency(LinkedList<Arc<Dependency>>),
    DependencyNotFound(Arc<Dependency>),
    DependencyAlreadyInstalled(Arc<Dependency>),
    DependencyNotInstalled(Arc<Dependency>),
    // 下载时遇到错误
    DownloadError(String),
    // 安装时遇到错误
    InstallDependencyError(String),
    // 解析依赖时遇到错误
    ParseDependencyError(String),
    // 删除依赖时遇到错误
    RemoveDependencyError(String),
    // software加锁失败
    SoftwareLockError(String),
}
// 有一个对软件包做管理的类，持有所有下载的软件包
// 垃圾回收，删除所有引用为0的软件包
pub struct SoftwareManager {
    softwares: Vec<Arc<Software>>,
    hashmap: HashMap<Arc<Dependency>, Arc<Software>>,
}
impl SoftwareManager {
    fn new() -> SoftwareManager {
        return SoftwareManager {
            softwares: Vec::new(),
            hashmap: HashMap::new(),
        };
    }
    fn hashmap(&self) -> &HashMap<Arc<Dependency>, Arc<Software>> {
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
        dependency: Arc<Dependency>,
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
        dependency_list: DependencyList,
    ) -> Result<LinkedList<Arc<Dependency>>, SoftwareManagerError> {
        // 用于判断是否已经下载或遍历时已经加入下载队列
        let mut hashmap: HashMap<Arc<Dependency>, bool> = HashMap::new();
        for key in self.hashmap.keys() {
            hashmap.insert(key.clone(), true);
        }
        // 用于记录下载队列，按顺序，无需其它依赖的排前面
        let mut download_list: LinkedList<Arc<Dependency>> = LinkedList::new();
        let mut hashset: HashSet<Arc<Dependency>> = HashSet::new();
        let mut linkedlist: LinkedList<Arc<Dependency>> = LinkedList::new();
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
        dependency: Arc<Dependency>,
        hashset: &mut HashSet<Arc<Dependency>>,
        linkedlist: &mut LinkedList<Arc<Dependency>>,
        hashmap: &mut HashMap<Arc<Dependency>, bool>,
        download_list: &mut LinkedList<Arc<Dependency>>,
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
        // todo 根据dep获得dep的依赖
        let config: DependencyList;
        match Self::get_dep(dependency.clone()) {
            Ok(dependency_list) => config = dependency_list,
            Err(err) => return Err(err),
        }
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
        download_list: LinkedList<Arc<Dependency>>,
    ) -> Result<(), SoftwareManagerError> {
        // 将下载依赖交给下载器依次下载
        for dependency in download_list.iter() {
            let result = download_unit().download(dependency.clone());
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
    pub fn update_reference(&mut self, list: DependencyList) -> Result<(), SoftwareManagerError>{
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
    // todo 查找依赖包的依赖
    pub fn get_dep(dep: Arc<Dependency>) -> Result<DependencyList, SoftwareManagerError> {
        // 本地测试
        let list = profile_handler().analyse("software-package/".to_string() + &dep.archive.clone() +"-" + &dep.version.version() + ".txt");
        //
        return Ok(list);
    }
}

//
lazy_static! {
    static ref DOWNLOAD_UNIT: Arc<DownloadUnit> = Arc::new(DownloadUnit::new());
}
//
#[inline(always)]
#[allow(dead_code)]
pub fn download_unit() -> &'static Arc<DownloadUnit> {
    &DOWNLOAD_UNIT
}
pub struct DownloadUnit {}
impl DownloadUnit {
    pub fn download(&self, dependency: Arc<Dependency>) -> Result<Arc<Software>, SoftwareManagerError> {
        // 创建一个新的 tokio 运行时环境
        let rt = Runtime::new().unwrap();    
        // 在异步上下文中执行异步函数并等待结果返回
        let result = rt.block_on(async {
            self.download_sync(dependency).await
        });
        return result;
    }
    // 同步
    pub async fn download_sync(&self, dependency: Arc<Dependency>) -> Result<Arc<Software>, SoftwareManagerError> {
        // 找到下载地址
        let downloadsite = dependency.download();
        // todo 下载到本地，本地路径暂不确定
        let path = "  ".to_string();
        //
        let client = reqwest::Client::new();
        let response ;
        match  client.get(downloadsite).send().await{
            Ok(r) => response = r,
            Err(err) => return Err(SoftwareManagerError::DownloadError(err.to_string())),
        };
        let file : Value = match response.text().await {
            Ok(f) => serde_json::from_str(&f).unwrap(),
            Err(err) => return Err(SoftwareManagerError::DownloadError(err.to_string())),
        };
        // 根据返回结果操作
        if file.get("status_code").is_some() {
            return Err(SoftwareManagerError::DownloadError(file.get("message").unwrap().to_string()));
        }
        if file.get("data").is_none() {
            return Err(SoftwareManagerError::DownloadError("data is none".to_string()));
        }
        // 获得返回结果，如果没有问题就安装到本地
        let data  = file.get("data").unwrap().as_str().unwrap();
        let decoded_data: Vec<u8> = match decode(data) {
            Ok(decoded) => decoded,
            Err(_) => {
                return Err(SoftwareManagerError::DownloadError("Failed to decode Base64 string.".to_string()));
            }
        };
        match install_unit().install(decoded_data) {
            Ok(o) => {},
            // todo 错误类型定义
            Err(err) => return Err(err),
        } 
        return Ok(Software::new(path, dependency));
    }
    // 异步
    // async fn download_async (){
    //     let body = reqwest::get("https://www.rust-lang.org").await?.text().await?;
    //     println!("body = {:?}", body);
    // }
    
    pub fn new() -> DownloadUnit {
        return DownloadUnit {};
    }
}

lazy_static! {
    static ref INSTALL_UNIT: Arc<InstallUnit> = Arc::new(InstallUnit::new());
}
//
#[inline(always)]
#[allow(dead_code)]
pub fn install_unit() -> &'static Arc<InstallUnit> {
    &INSTALL_UNIT
}
// 安装模块
pub struct InstallUnit {}
impl InstallUnit {
    
    pub fn new() -> InstallUnit {
        return InstallUnit {};
    }
    pub fn install(&self, decoded_data : Vec<u8>) -> Result<(), Error>{
        let compressed_data: Vec<u8> = decoded_data;
        // 将数据写入文件
        let mut file = File::create("output.tar")?;
        file.write_all(&compressed_data)?;
        return Ok(());
    }
}
