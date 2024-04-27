use std::{collections::HashMap, fmt, hash::{Hash, Hasher}, ops::Deref, str::FromStr, sync::{Arc, Mutex, MutexGuard}};
use lazy_static::lazy_static;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use crate::{entity::dependency::{Configuration, Package, PackageList}, tool::network_module::download_unit};
use crate::entity::software::{Software};
use crate::entity::version_wrapper::VersionWrapper;

// 


pub enum PackageManagerError {
    // Configuration重复存在
    DuplicateConfiguration,
    // 配置不存在
    ConfigurationNotFound(String),
    // 更新失败
    ConfigurationUpdateFailed,
    // 加锁失败
    ConfigurationLockFailed,
}
// 底层包管理器作为单例
lazy_static! {
    static ref PACKAGE_MANAGER : Arc<Mutex<PackageManager>> = Arc::new(Mutex::new(PackageManager::new()));
}
#[inline(always)]
#[allow(dead_code)]
pub(super) fn package_manager() -> &'static Arc<Mutex<PackageManager>> {
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
    package_hashmap : HashMap<str, Arc<Package>>,
    
}
impl PackageManager {
    fn new() -> PackageManager {
        // todo 初始化时检查数据文件，没有则创建，有则根据文件恢复数据
        return PackageManager{ packages : Vec::new() ,  package_hashmap : HashMap::new()};
    }
    pub fn install_package(&mut self, package : Arc<Package>) -> Result<(),PackageManagerError> {
        let path = "/database";
        // 调用下载器下载包并解压
        match download_unit().download_software(package, path) {
            Ok(_) => {
                self.packages.push(package);
                self.package_hashmap.insert(package.to_string(), package);
            },
            Err(e) => {
                return Err(e);
            }
        }
        // todo 执行安装脚本
        // doto 添加到数据文件中
        return Ok(());
    }

    pub fn uninstall_package(&mut self, archive : String, version : VersionWrapper) -> Result<(),PackageManagerError> {
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
            return Err(PackageManagerError::ConfigurationLockFailed);
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