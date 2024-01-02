use std::{collections::{HashMap, HashSet, LinkedList}, sync::{Arc, Mutex}, result};
use lazy_static::lazy_static;
use serde::de;
use simple_logger::SimpleLogger;
use crate::dep_manager::{Dependency, Dependency_List};

// 安装的软件包
#[derive(Clone)]
pub struct Software
{
    // 指针，指向实际安装位置
    ptr : String,
    // 引用计数。 包括new version和 last version
    reference_count : i32,
    // 
    dependency : Dependency
}
impl Software {
    pub fn new (path : String, dependency : Dependency) -> Software{
        return Software{
            ptr : path,
            reference_count : 0,
            dependency : dependency
        };
    }
    pub fn reference_count(&self) -> i32 {
        return self.reference_count;
    }
    pub fn dependency(&self) -> &Dependency {
        return &self.dependency;
    }
    // 对软件包的引用计数做修改
}

// 管理器作为单例
lazy_static! {
    static ref SOFTWARE_MANAGER : Arc<Mutex<SoftwareManager>> = Arc::new(Mutex::new(SoftwareManager::new()));
}
#[inline(always)]
#[allow(dead_code)]
pub(super) fn software_manager() -> &'static Arc<Mutex<SoftwareManager>> {
    &SOFTWARE_MANAGER
}

pub enum SoftwareManagerError {
    CircularDependency(LinkedList<Dependency>),
    DependencyNotFound(Dependency),
    DependencyAlreadyInstalled(Dependency),
    DependencyNotInstalled(Dependency),
    // 下载时遇到错误
    DownloadvError(String),
    // 安装时遇到错误
    InstallDependencyError(String),
    // 解析依赖时遇到错误
    ParseDependencyError(String),
    // 删除依赖时遇到错误
    RemoveDependencyError(String),
}
// 有一个对软件包做管理的类，持有所有下载的软件包
// 垃圾回收，删除所有引用为0的软件包
pub struct SoftwareManager{
    softwares : Vec<Software>,
    hashmap : HashMap<Dependency, bool>,
}
impl SoftwareManager {
    fn new () -> SoftwareManager {
        return SoftwareManager{
            softwares : Vec::new(),
            hashmap : HashMap::new(),
        }
    }
    // 删除所有引用数为0的依赖包
    pub fn garbage_collection(&mut self) -> Result<(), SoftwareManagerError>{
        let mut result = Ok(());
        // 检查引用是否为0 ，为0则删除文件，再从数组移除
        let mut indexes_to_remove = Vec::new();
        // 使用 for 循环找到要删除的元素的索引
        for (index, soft) in self.softwares.iter().enumerate() {
            if  soft.reference_count() == 0 {
                // todo 删除文件
                // if {
                //     result = Err(SoftwareManagerError::RemoveDependencyError("".to_string()));
                //     break;
                // }
                
                indexes_to_remove.push(index);
            }
        }
        // 根据索引删除元素
        for index in indexes_to_remove.iter().rev() {
            self.softwares.remove(*index);
        }
        log::info!("garbage collection success");
        return result;
    }
    // 添加一个新软件
    pub fn insert(&mut self, software: Software, dependency : Dependency) ->Result<(),SoftwareManagerError> {
        if !self.hashmap.contains_key(&dependency) {
            self.softwares.push(software);
            self.hashmap.insert(dependency, true);
        }else {
            return Err(SoftwareManagerError::DependencyAlreadyInstalled(dependency));
        }
        return Ok(());
    }
    // 检查dependency_list是否存在环
    pub fn check(& self, dependency_list : Dependency_List) -> Result<LinkedList<Dependency>,SoftwareManagerError>{
        // 用于判断是否已经下载或遍历时已经加入下载队列
        let mut hashmap = self.hashmap.clone();
        // 用于记录下载队列，按顺序，无需其它依赖的排前面
        let mut download_list : LinkedList<Dependency> = LinkedList::new();
        let mut hashset : HashSet<Dependency> = HashSet::new();
        let mut linkedlist : LinkedList<Dependency> = LinkedList::new();
        for dependency in dependency_list.dependencies {
            // 检测出环形
            match Self::dfs(self, dependency, &mut hashset, &mut linkedlist, &mut hashmap, &mut download_list) {
                Ok(b) => match b {
                    true => {},
                    false => return Err(SoftwareManagerError::CircularDependency(download_list))
                }
                Err(e) => return Err(e),
            }
            
        }
        return Ok(download_list);
    }
    
    // 深度优先搜索查找环
    fn dfs(& self, dep : Dependency,   hashset :  &mut HashSet<Dependency>,  linkedlist :  &mut LinkedList<Dependency>
    , hashmap :  &mut HashMap<Dependency,bool>, download_list :  &mut LinkedList<Dependency>) -> Result<bool, SoftwareManagerError>{
        //
        let dependency = &dep;
        if hashmap.contains_key(&dependency) {
            return Ok(true);
        }
        if !hashset.insert(dependency.clone()) {
            // 出现环形，用download list 装环形
            download_list.clear();
            for dep_tmp in linkedlist.iter().rev() {
                //println!("{} {} -> ", dep_tmp.archive, dep_tmp.version);
                download_list.push_back(dep_tmp.clone());
                if dep_tmp == dependency{
                    break;
                }
            }
            return Ok(false);
        }
        linkedlist.push_back(dependency.clone());
        // todo 根据dep获得dep的依赖
        let config : Dependency_List;
        match Self::get_dep(dependency.clone()){
            Ok(dependency_list) => config = dependency_list,
            Err(err) => return Err(err),
        }
        for dependency in config.dependencies {
            match self.dfs( dependency, hashset, linkedlist, hashmap, download_list) {
                Ok(b) => match b {
                    true => (),
                    false => return Ok(false),
                },
                Err(err) => return Err(err),
            }
        }
        linkedlist.pop_back();
        hashset.remove(dependency);
        hashmap.insert(dependency.clone(), true);
        download_list.push_front(dependency.clone());
        return Ok(true);
    }

    // 根据下载队列来安装
    pub fn install (&mut self, download_list :  LinkedList<Dependency>) ->Result<(), SoftwareManagerError> {
        // 将下载依赖交给下载器依次下载
        for dependency in download_list.iter() {
            let result = download_unit().download(dependency);
            let software;
            match result {
                Ok(s) => software = s,
                Err(err) => return Err(err),
            }
            // 加入新下载的依赖包
            self.insert(software, dependency.clone());
        }
        return Ok(());
    }

    // todo 查找依赖包的依赖
    pub fn get_dep(dep : Dependency) -> Result<Dependency_List, SoftwareManagerError>{

        return Ok(Dependency_List::new(Vec::new()));
    }
}

// 
const DOWNLOAD_SITE: &str = "www.";

// 
lazy_static! {
    static ref DOWNLOAD_UNIT : Arc<DownloadUnit>= Arc::new(DownloadUnit::new());
}
//
#[inline(always)]
#[allow(dead_code)]
pub fn download_unit() -> &'static Arc<DownloadUnit> {
    &DOWNLOAD_UNIT
}
pub struct DownloadUnit {
    
}
impl DownloadUnit {
    pub fn download(& self, dependency : &Dependency) -> Result<Software, SoftwareManagerError>{
        // 找到下载地址
        let downloadsite = dependency.download();
        // todo 下载到本地
        let path = "  ".to_string();
        return Ok(Software::new(path, dependency.clone()));
    }
    // 同步
    fn download_sync () -> Result<(), reqwest::Error>{
        let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;
        println!("body = {:?}", body);
        Ok(())
    }
    // 异步
    // async fn download_async (){
    //     let body = reqwest::get("https://www.rust-lang.org").await?.text().await?;
    //     println!("body = {:?}", body);
    // }
    pub fn new () -> DownloadUnit{
        return DownloadUnit {  };
    }
    
    
}