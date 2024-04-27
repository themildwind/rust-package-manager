
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

use crate::{entity::dependency::{Dependency, Package, PackageList}, tool::network_module::download_unit};
use crate::entity::software::{Software};
use crate::entity::version_wrapper::VersionWrapper;

use super::package_manager::{self, package_manager};
// 安装的软件包


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
    CircularDependency(LinkedList<Arc<Package>>),
    DependencyNotFound(Arc<Package>),
    DependencyAlreadyInstalled(Arc<Package>),
    DependencyNotInstalled(Arc<Package>),
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
    softwares_hashmap: HashMap<String, Arc<Software>>,
}
impl SoftwareManager {
    fn new() -> SoftwareManager {
        // todo 作为一个上层管理器，包括依赖解决、引用计数，更新软件从这里开始，安装软件由次获得依赖关系，删除软件由此获得哪些包要被删除
        // todo 在这里用数据文件保存已存在的包的引用计数，每个包最新的版本
        return SoftwareManager {
            softwares: Vec::new(),
            softwares_hashmap: HashMap::new(),
        };
    }
    fn hashmap(&self) -> &HashMap<str, Arc<Software>> {
        return &self.softwares_hashmap;
    }
    // 删除所有引用数为0的依赖包
    pub fn garbage_collection(&mut self) -> Result<(), SoftwareManagerError> {
        // 检查引用是否为0 ，为0则删除文件，再从数组移除
        for software in self.softwares.iter() {
            let inner_guard = software.inner();
            if inner_guard.is_some() {
                let mut guard = inner_guard.unwrap();
                if guard.reference_count() == 0 {
                    // 调用迭代算法，删除无用文件
                    let package_manager_guard = package_manager().lock().unwrap();
                    self.remove_software(software, &package_manager_guard).unwrap();
                }
            }
            else {
                return Err(SoftwareManagerError::SoftwareLockError(software.archive.clone()));
            }
        }
        // todo 根据操作之后的结果，生成新的数据文件覆盖原来的
        return Ok(());
    }
    // 迭代卸载
    fn remove_software(&mut self, software: &Arc<Software>, package_manager_guard: &MutexGuard<package_manager::PackageManager>) -> Result<(), SoftwareManagerError> {
        // 调用底层管理器删除文件
        match package_manager_guard.uninstall_package(software.archive, software.version.clone()) {
            Ok(_) => {},
            Err(err) => return Err(SoftwareManagerError::RemoveDependencyError(err)),
        };
        // 修改数据文件信息
        let inner_guard = software.inner();
        if inner_guard.is_some() {
            let mut guard = inner_guard.unwrap();
            guard.remove();
        }
        else {
            return Err(SoftwareManagerError::SoftwareLockError(software.archive.clone()));
        }
        // 遍历它的依赖包
        for dep in software.dependencies.iter() {
            let dependency_software = match self.softwares_hashmap.get(&dep.to_string()) {
                Some(s) => s,
                None => continue
            };           
            let dep_soft_inner_guard = dependency_software.inner();
            if dep_soft_inner_guard.is_some() {
                let mut dep_guard = dep_soft_inner_guard.unwrap();
                // 减少引用计数，并判断是否为0，是则继续删除这个包
                if dep_guard.descrease() == 0 {
                    match self.remove_software(dependency_software, package_manager_guard) {
                        Ok(_) => {},
                        Err(err) => return Err(err),
                    }
                }
            }
        }
        // 
        return Ok(());
    }
    // 下载新软件
    pub fn install_package(
        &mut self,
        dependencies: Vec<Arc<Dependency>>,
    ) -> Result<(), SoftwareManagerError> {
        // 检查
        let download_list = match self.check(dependencies) {
            Ok(list) => list,
            Err(err) => return Err(err),
        };
        let package_manager_guard = package_manager().lock().unwrap();
        // 下载依赖包
        for package in download_list.iter() {
            // 下载依赖包
            match package_manager_guard.install_package(package) {
                Ok(_) => {},
                Err(err) => return Err(SoftwareManagerError::DownloadError(err)),
            }
        }
        // doto 修改数据文件
        return Ok(());
    }
    // 检查dependency_list是否存在环
    pub fn check(
        &self,
        dependency_list: Vec<Dependency>,
    ) -> Result<LinkedList<Arc<Package>>, SoftwareManagerError> {
        // 用于判断是否已经下载或遍历时已经加入下载队列
        let mut hashmap: HashMap<Arc<Package>, bool> = HashMap::new();
        for key in self.softwares_hashmap.keys() {
            hashmap.insert(key.clone(), true);
        }
        // 用于记录下载队列，按顺序，无需其它依赖的排前面
        let mut download_list: LinkedList<Arc<Package>> = LinkedList::new();
        let mut hashset: HashSet<Arc<Package>> = HashSet::new();
        let mut linkedlist: LinkedList<Arc<Package>> = LinkedList::new();
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
        dependency: Arc<Package>,
        hashset: &mut HashSet<Arc<Package>>,
        linkedlist: &mut LinkedList<Arc<Package>>,
        hashmap: &mut HashMap<Arc<Package>, bool>,
        download_list: &mut LinkedList<Arc<Package>>,
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
    // 更新引用计数
    pub fn update_reference(&mut self, list: PackageList) -> Result<(), SoftwareManagerError>{
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
}


