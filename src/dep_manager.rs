use std::{collections::HashMap, fmt, ops::Deref};

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
// 实现 Deref trait，指定解引用的行为
// impl Deref for Dependency {
//     type Target = (String, String, String, String, String, String, String, String); // 选择你希望被解引用的字段的类型

//     fn deref(&self) -> &Self::Target {
//         // 返回元组，包含需要被解引用的字段
//         &(&self.archive, &self.version, &self.component, &self.origin, &self.label, &self.architecture, &self.download, &self.others)
//     }
// }
// 实现 Debug trait
// impl fmt::Debug for Dependency {
//     // 实现 fmt 方法
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // 使用 write! 宏将结构体的字段格式化到 f 中
//         write!(f, "Dependency {{ archive: {}, version: {} }}", self.archive, self.version)
//     }
// }
// 表示一个程序的所依赖的软件包集合
// 每个应用程序所有，会根据配置文件构建
// 并且构建版本链
#[derive(Clone, Debug,Deserialize, PartialEq, Eq,Hash,Serialize)]
pub struct Dependency_List{
    // 一个数组，表示所有依赖的包
    pub dependencies : Vec<Dependency>,
    
}
impl Dependency_List{
    pub fn new(vec : Vec<Dependency>) -> Dependency_List{
        return Dependency_List{
            dependencies : vec,
        };
    }
    // 接受一个包含依赖的 Vec<Dependency> 创建新的配置
    pub fn with_dependencies(vec: Vec<Dependency>) -> Dependency_List{
        Dependency_List{
            dependencies: vec,
        }
    }
}

#[derive(Clone, Debug,PartialEq, Eq,Hash)]
// 表示一个程序的配置文件。一对一
pub struct Configuration{
    // 持有所有的版本
    pub configs : Vec<Dependency_List>,
    // 标记这个配置文件
    pub archive: String,
    pub version: Version,
}
impl  Configuration  {
    // 更新指定依赖包
    // fn update(&mut self, software_name : String,new_version : Version) -> Configuration{
    //     let mut tmp = self.config.clone();
    //     // 遍历找到对应的软件包
    //     for  depend in tmp.dependencies {
    //         if depend.archive == software_name{
    //             //depend.update(new_version);
    //             break;
    //         }
    //     }
    //     // 返回一个新的configuration对象
    //     //tmp.last = self.config;
    //     return tmp;
    // }

    // 全部更新
    fn all_update(&mut self) -> Dependency_List{
        let tmp = self.configs.last().unwrap().clone();
        return tmp;
    }
    // 传入依赖列表
    pub fn new(list : Dependency_List, archive : String, version : Version) -> Configuration{
        let mut vec : Vec<Dependency_List> = Vec::new();
        vec.push(list);
        return Configuration{
            configs : vec,
            archive : archive,
            version : version,
        };
    }
}

// 配置文件管理器作为单例
lazy_static! {
    static ref CONFIGURATION_MANAGER : ConfigurationManager = ConfigurationManager::new();
}
pub struct ConfigurationManager {
    // 保存所有程序的配置文件
    configurations : Vec<Configuration>,
    // 用map记录
    hashmap : HashMap<Configuration, bool>
}
impl ConfigurationManager {
    fn new() -> ConfigurationManager {
        return ConfigurationManager{ configurations :  Vec::new() , hashmap : HashMap::new()};
    }
    // &mut self 这样的引用使得方法可以访问修改调用者对象的数据
    fn insert(&mut self, configuration : Configuration) {
        if !self.hashmap.contains_key(&configuration){
            self.configurations.push(configuration);
        }
        
    }
    fn delete(&mut self, configuration : Configuration) {
        if self.hashmap.contains_key(&configuration){
            self.configurations.retain(|config| config != &configuration);
        }
        
    }
}