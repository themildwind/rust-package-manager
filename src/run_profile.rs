use std::fs;
use serde_derive::{Deserialize, Serialize};

use crate::{dep_manager::{Dependency, Configuration, DependencyList}, version_mod::Version};
extern crate toml;

#[inline(always)]
#[allow(dead_code)]
pub fn profile_handler() -> &'static ProfileHandler {
    &ProfileHandler
}
// 一个仅仅用于解析toml格式的类
#[derive(Clone, Debug,Deserialize,Serialize)]
pub struct DependencyListTomlHandler{
    pub dependencies : Vec<Dependency>,
}
// 作为一个工具类，用来处理配置文件
#[derive(Clone, Debug)]
pub struct ProfileHandler;

impl ProfileHandler{
    
    
    // 读取txt文件解析Template
    pub fn analyse (&self, path : String) -> Configuration{
        // 读取 TOML 文件内容
        let toml_content = fs::read_to_string(path).expect("Unable to read file");
        //println!("{:?}", toml_content);
        // 解析 TOML 格式数据
        let vec: DependencyListTomlHandler = toml::from_str(&toml_content).expect("Failed to parse TOML");
        let dependency_list = DependencyList::new(vec.dependencies.clone());
        println!("{:?}", dependency_list);
        // 初始化一个新的Configuratio
        let configuration = Configuration::new(dependency_list, "待实现".to_string(), 0);
        // 完成一个程序的配置文件的解析，接下来交给后续管理器检查下载
        return configuration;
    }
}