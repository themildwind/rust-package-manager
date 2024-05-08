use std::{collections::HashMap, fmt, hash::{Hash, Hasher}, ops::Deref, str::FromStr, sync::{Arc, Mutex, MutexGuard}};
use lazy_static::lazy_static;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use crate::{entity::dependency::{self, Configuration, Dependency, Package, PackageList}, tool::{network_module::download_unit, resolve_file::profile_handler}};
use crate::entity::software::{Software};
use crate::entity::version_wrapper::VersionWrapper;

// 


pub enum PackageManagerError {
    // 重复存在
    PackageInstalled,
    // 不存在
    PackageNotFound(String),
    // 加锁失败
    PackageLockFailed,
    // 安装失败
    PackageInstallFailed,
    // 卸载失败
    PackageUninstallFailed,
    // 读取本地文件错误
    ReadLocalPackageFileError(String),
}
impl PackageManagerError {
    pub fn to_string(&self) -> String {
        match self {
            PackageManagerError::PackageInstalled => {
                return "Package already installed".to_string();
            },
            PackageManagerError::PackageNotFound(s) => {
                return format!("Package {} not found", s);
            },
            PackageManagerError::PackageLockFailed => {
                return "Package lock failed".to_string();
            },
            PackageManagerError::PackageInstallFailed => {
                return "Package install failed".to_string();
            }
            PackageManagerError::PackageUninstallFailed => {
                return "Package uninstall failed".to_string();
            }
            PackageManagerError::ReadLocalPackageFileError(s) => {
                return format!("Read local package file error {}", s);
            }

        }
    }
}
// 底层包管理器作为单例
lazy_static! {
    static ref PACKAGE_MANAGER : Arc<Mutex<PackageManager>> = Arc::new(Mutex::new(PackageManager::new()));
}
#[inline(always)]
#[allow(dead_code)]
pub fn package_manager() -> &'static Arc<Mutex<PackageManager>> {
    &PACKAGE_MANAGER
}
// 内部类
pub enum VersionMode {
    // 最新版本
    Latest,
    // 指定版本
    Specific(VersionWrapper),
}
// 用于表示更新的模式
pub enum ConfigurationUpdateMode {
    // 全部更新最新
    AllUpdateLatest,
    // 指定更新策略
    PartialUpdate(HashMap<String,VersionMode>), 
}
pub struct PackageManager {
    // 保存所有包
    packages : Vec<Arc<Package>>,
    // 用map记录
    package_hashmap : HashMap<String, Arc<Package>>,
    
}
impl PackageManager {
    fn new() -> PackageManager {
        // 初始化时检查数据文件，没有则创建，有则根据文件恢复数据
        let path = "database/package_data.toml";
        let packages = match profile_handler().analyse_package_file(path.to_string()) {
            Ok(s) => s,
            Err(err) => {
                panic!("{}",err.to_string());
            }
        };
        let mut map : HashMap<String, Arc<Package>> = HashMap::new();
        for package in packages.iter() {
            let str = format!("{}-{}", package.archive, package.version_wrapper.to_string());
            map.insert(str, package.clone());
        }
        return PackageManager{ packages : packages ,  package_hashmap : map};
    }
    fn get_package(&self, dependency: Arc<Dependency>) -> Result<Arc<Package>, PackageManagerError> {
        // 网络获取详细信息
        return download_unit().get_package_information(dependency);
    }
    pub fn install_package(&mut self, dependency: Arc<Dependency>) -> Result<(),PackageManagerError> {
        // 安装地址
        let path = "/database";
        let package = match self.get_package(dependency.clone()) {
            Ok(p) => p,
            Err(e) => {
                return Err(e);
            }
        };
        // 调用下载器下载包并解压
        match download_unit().download_software(package.clone(), path) {
            Ok(_) => {
                self.packages.push(package.clone());
                self.package_hashmap.insert(package.to_string(), package);
                // doto 添加到数据文件中
            },
            Err(e) => {
                return Err(PackageManagerError::PackageInstallFailed);
            }
        }
        // todo 执行安装脚本
        
        return Ok(());
    }

    pub fn uninstall_package(& self, archive : String, version : VersionWrapper) -> Result<(),PackageManagerError> {
        // todo 卸载软件，执行卸载脚本，删除文件
        // todo 修改数据文件
        return Ok(());
    }
    // 加入新的配置文件。一般是在第一次解析时添加
    // pub fn insert(&mut self, configuration : Arc<Configuration>) -> Result<(), PackageManagerError>{
    //     if !self.config_hashmap.contains_key(&configuration){
    //         self.configurations.push(configuration.clone());
    //         self.config_hashmap.insert(configuration.clone(), true);
    //         self.archive_hashmap.insert(configuration.archive.clone(), configuration.clone());
    //     }else {
    //         return Err(PackageManagerError::DuplicateConfiguration);
    //     }
    //     Ok(())
    // }
    // // 
    // pub fn delete(&mut self, configuration : Configuration) -> Result<(), PackageManagerError>{
    //     if self.config_hashmap.contains_key(&configuration){
    //         self.configurations.retain(|config| !configuration.eq(&config));
    //     }else {
    //         return Err(PackageManagerError::ConfigurationNotFound(configuration.archive.clone()));
    //     }
    //     Ok(())
    // }

    // // 更新某个配置
    // pub fn update(&mut self, archive : String, mode : ConfigurationUpdateMode) -> Result<PackageList,PackageManagerError>{
    //     if !self.archive_hashmap.contains_key(&archive) {
    //         // 配置文件不存在
    //         return Err(PackageManagerError::ConfigurationNotFound(archive.clone()));
    //     }
    //     let config = self.archive_hashmap.get(&archive).unwrap();
    //     // 传入引用，实例被修改，添加新的依赖列表
    //     let list : PackageList = match configuration_update_unit().get_new_configuration(config.clone(), mode) {
    //         Ok(l) => l,
    //         Err(e) => return Err(e),
    //     };
    //     return Ok(list);
    // }
}


//
lazy_static! {
    static ref CONFIGURATION_UPDATE_UNIT: Arc<ConfigurationUpdateUnit> = Arc::new(ConfigurationUpdateUnit::new());
}
//
#[inline(always)]
#[allow(dead_code)]
pub fn configuration_update_unit() -> &'static Arc<ConfigurationUpdateUnit> {
    &CONFIGURATION_UPDATE_UNIT
}
pub struct ConfigurationUpdateUnit;
impl ConfigurationUpdateUnit {
    pub fn new() -> ConfigurationUpdateUnit {
        ConfigurationUpdateUnit
    }
    pub fn get_new_configuration(&self, configuration : Arc<Configuration>, mode : ConfigurationUpdateMode) -> Result<PackageList,PackageManagerError>{
        let inner_guard = configuration.inner();
        if inner_guard.is_none() {
            return Err(PackageManagerError::PackageLockFailed);
        }
        let guard = inner_guard.unwrap();
        // todo 上网更新
        let mut list = PackageList::new(Vec::new());
        if let tmp = guard.vec().last().is_some() {
            list = guard.vec().last().unwrap().clone();
        }
        // 表示更新成功
        return Ok(list);
    }
}