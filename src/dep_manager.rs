use std::{collections::HashMap, fmt, ops::Deref, sync::{Arc, Mutex}};
use serde_derive::{Deserialize, Serialize};
use lazy_static::lazy_static;
use crate::{version_mod::Version, software_manager::Software};
// 
#[derive(Clone, Debug, Deserialize,Serialize,PartialEq, Eq,Hash)]
pub struct Dependency
{
    pub archive: String,
    pub component: String,
    pub origin: String,
    pub label: String,
    pub architecture: String,
    pub download : String,
    pub others: String,
    pub version: Version,
}
impl Dependency{
    fn new () -> Dependency{
        return Dependency{
            archive: String::new(),
            version: Version::new("0".to_string()),
            component: String::new(),
            origin: String::new(),
            label: String::new(),
            architecture: String::new(),
            download : String::new(),
            others: String::new(),
        };
    }
    pub fn download(&self) -> String{
        return self.download.clone();
    }
}

// 表示一个程序的所依赖的软件包集合
// 每个应用程序所有，会根据配置文件构建
// 并且构建版本链
#[derive(Clone, Debug, PartialEq, Eq,Hash)]
pub struct DependencyList{
    // 一个数组，表示所有依赖的包
    pub dependencies : Vec<Arc<Dependency>>,
}
impl DependencyList{
    pub fn new(vec : Vec<Dependency>) -> DependencyList{
        return  DependencyList{
            dependencies : vec
        .into_iter()
        .map(|dep| Arc::new(dep))
        .collect()
        }
    }
    // 接受一个包含依赖的 Vec<Dependency> 创建新的配置
    pub fn with_dependencies(vec: Vec<Arc<Dependency>>) -> DependencyList{
        DependencyList{
            dependencies: vec,
        }
    }
}

#[derive(Clone,Debug,PartialEq, Eq,Hash)]
// 表示一个程序的配置文件。一对一
pub struct Configuration{
    // 持有所有的版本
    pub configs : Vec<DependencyList>,
    // 标记这个配置文件
    pub archive: String,
    pub age: usize,
}

impl  Configuration  {
    // 传入依赖列表
    pub fn new(list : DependencyList, archive : String, age : usize) -> Configuration{
        let mut vec : Vec<DependencyList> = Vec::new();
        vec.push(list);
        return Configuration{
            configs : vec,
            archive : archive,
            age : age,
        };
    }
}

pub enum ConfigurationManagerError {
    // Configuration重复存在
    DuplicateConfiguration,
    // 配置不存在
    ConfigurationNotFound(String),
    // 更新失败
    ConfigurationUpdateFailed,
}
// 配置文件管理器作为单例
lazy_static! {
    static ref CONFIGURATION_MANAGER : Arc<Mutex<ConfigurationManager>> = Arc::new(Mutex::new(ConfigurationManager::new()));
}
#[inline(always)]
#[allow(dead_code)]
pub(super) fn configuration_manager() -> &'static Arc<Mutex<ConfigurationManager>> {
    &CONFIGURATION_MANAGER
}
// 内部类
pub enum VersionMode {
    // 最新版本
    Latest,
    // 指定版本
    Specific(Version),
}
// 用于表示更新的模式
pub enum ConfigurationUpdateMode {
    // 全部更新最新
    AllUpdateLatest,
    // 指定更新策略
    PartialUpdate(HashMap<String,VersionMode>), 
}
pub struct ConfigurationManager {
    // 保存所有程序的配置文件
    configurations : Vec<Configuration>,
    // 用map记录
    config_hashmap : HashMap<Configuration, bool>,
    archive_hashmap : HashMap<String, Configuration>,
}
impl ConfigurationManager {
    fn new() -> ConfigurationManager {
        return ConfigurationManager{ configurations :  Vec::new() , config_hashmap : HashMap::new(), archive_hashmap : HashMap::new()};
    }
    // 加入新的配置文件。一般是在第一次解析时添加
    pub fn insert(&mut self, configuration : Configuration) -> Result<(), ConfigurationManagerError>{
        if !self.config_hashmap.contains_key(&configuration){
            self.configurations.push(configuration.clone());
            self.config_hashmap.insert(configuration.clone(), true);
            self.archive_hashmap.insert(configuration.archive.clone(), configuration.clone());
        }else {
            return Err(ConfigurationManagerError::DuplicateConfiguration);
        }
        Ok(())
    }
    // 
    pub fn delete(&mut self, configuration : Configuration) -> Result<(), ConfigurationManagerError>{
        if self.config_hashmap.contains_key(&configuration){
            self.configurations.retain(|config| config != &configuration);
        }else {
            return Err(ConfigurationManagerError::ConfigurationNotFound(configuration.archive.clone()));
        }
        Ok(())
    }

    // 更新某个配置
    pub fn update(&mut self, archive : String, mode : ConfigurationUpdateMode) -> Result<Configuration,ConfigurationManagerError>{
        if !self.archive_hashmap.contains_key(&archive) {
            // 配置文件不存在
            return Err(ConfigurationManagerError::ConfigurationNotFound(archive.clone()));
        }
        let config = self.archive_hashmap.get(&archive).unwrap();
        let new_config ;
        match configuration_update_unit().get_new_configuration(config.clone(), mode) {
            Ok(c) => new_config = c,
            Err(e) => return Err(e),
        }
        return Ok(new_config);
    }
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
    pub fn get_new_configuration(&self, configuration : Configuration, mode : ConfigurationUpdateMode) -> Result<Configuration,ConfigurationManagerError>{
        let mut new_configuration = configuration.clone();
        // todo 上网更新
        new_configuration.age = configuration.age + 1;
        return Ok(new_configuration);
    }
}