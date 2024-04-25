use std::{collections::HashMap, fmt, hash::{Hash, Hasher}, ops::Deref, str::FromStr, sync::{Arc, Mutex, MutexGuard}};
use lazy_static::lazy_static;
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use crate::{software_manager::Software, version_wrapper::VersionWrapper};

// 
#[derive(Clone, Debug, Deserialize,Serialize,PartialEq, Eq,Hash)]
pub struct DependencyItem
{
    pub archive: String,
    pub component: String,
    pub origin: String,
    pub label: String,
    pub architecture: String,
    pub download : String,
    pub others: String,
    pub version_wrapper: VersionWrapper,
}
impl DependencyItem{
    fn new () -> DependencyItem{
        return DependencyItem{
            archive: String::new(),
            version_wrapper: VersionWrapper::new(Version::new(0, 0, 0)),
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
pub struct DependencyItemList{
    // 一个数组，表示所有依赖的包
    pub dependencies : Vec<Arc<DependencyItem>>,
}
impl DependencyItemList{
    pub fn new(vec : Vec<DependencyItem>) -> DependencyItemList{
        return  DependencyItemList{
            dependencies : vec
        .into_iter()
        .map(|dep| Arc::new(dep))
        .collect()
        }
    }
    // 接受一个包含依赖的 Vec<Dependency> 创建新的配置
    pub fn with_dependencies(vec: Vec<Arc<DependencyItem>>) -> DependencyItemList{
        DependencyItemList{
            dependencies: vec,
        }
    }
}

#[derive(Debug)]
// 表示一个程序的配置文件。一对一
pub struct Configuration{
    
    // 标记这个配置文件
    pub archive: String,
    inner : Mutex<InnerConfiguration>
}
#[derive(Debug, Clone,PartialEq, Eq,Hash)]
pub struct InnerConfiguration {
    // 持有所有的版本
    pub vec : Vec<DependencyItemList>,
    pub age: usize,
}
impl InnerConfiguration{
    pub fn age(&self) -> usize {
        return self.age;
    }
    pub fn vec(&self) -> Vec<DependencyItemList> {
        return self.vec.clone();
    }
    // 
    fn add(&mut self) {
        self.age += 1;
    }
    pub fn update(&mut self, list: DependencyItemList) {
        self.vec.push(list);
        self.add();
    }
}
// 实现 PartialEq 和 Eq trait
impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        self.archive == other.archive
    }
}

impl Eq for Configuration {}
//
// 实现 Hash trait
impl Hash for Configuration {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 将 key 和 value 组合起来计算哈希值
        self.archive.hash(state);
    }
}
impl  Configuration  {
    // 传入依赖列表
    pub fn new(list : DependencyItemList, archive : String, age : usize) -> Arc<Configuration>{
        let mut vec : Vec<DependencyItemList> = Vec::new();
        vec.push(list);
        return Arc::new(Configuration{
            archive : archive,
            inner : Mutex::new(InnerConfiguration{
                vec : vec,
                age : age,
            })
        });
    }

    pub fn inner(&self) -> Option<MutexGuard<InnerConfiguration>>{
        self.inner.lock().ok()
    }
}

pub enum ConfigurationManagerError {
    // Configuration重复存在
    DuplicateConfiguration,
    // 配置不存在
    ConfigurationNotFound(String),
    // 更新失败
    ConfigurationUpdateFailed,
    // 加锁失败
    ConfigurationLockFailed,
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
    Specific(VersionWrapper),
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
    configurations : Vec<Arc<Configuration>>,
    // 用map记录
    config_hashmap : HashMap<Arc<Configuration>, bool>,
    archive_hashmap : HashMap<String, Arc<Configuration>>,
}
impl ConfigurationManager {
    fn new() -> ConfigurationManager {
        return ConfigurationManager{ configurations :  Vec::new() , config_hashmap : HashMap::new(), archive_hashmap : HashMap::new()};
    }
    // 加入新的配置文件。一般是在第一次解析时添加
    pub fn insert(&mut self, configuration : Arc<Configuration>) -> Result<(), ConfigurationManagerError>{
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
            self.configurations.retain(|config| !configuration.eq(&config));
        }else {
            return Err(ConfigurationManagerError::ConfigurationNotFound(configuration.archive.clone()));
        }
        Ok(())
    }

    // 更新某个配置
    pub fn update(&mut self, archive : String, mode : ConfigurationUpdateMode) -> Result<DependencyItemList,ConfigurationManagerError>{
        if !self.archive_hashmap.contains_key(&archive) {
            // 配置文件不存在
            return Err(ConfigurationManagerError::ConfigurationNotFound(archive.clone()));
        }
        let config = self.archive_hashmap.get(&archive).unwrap();
        // 传入引用，实例被修改，添加新的依赖列表
        let list : DependencyItemList = match configuration_update_unit().get_new_configuration(config.clone(), mode) {
            Ok(l) => l,
            Err(e) => return Err(e),
        };
        return Ok(list);
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
    pub fn get_new_configuration(&self, configuration : Arc<Configuration>, mode : ConfigurationUpdateMode) -> Result<DependencyItemList,ConfigurationManagerError>{
        let inner_guard = configuration.inner();
        if inner_guard.is_none() {
            return Err(ConfigurationManagerError::ConfigurationLockFailed);
        }
        let guard = inner_guard.unwrap();
        // todo 上网更新
        let mut list = DependencyItemList::new(Vec::new());
        if let tmp = guard.vec().last().is_some() {
            list = guard.vec().last().unwrap().clone();
        }
        // 表示更新成功
        return Ok(list);
    }
}