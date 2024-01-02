use std::fs;
use crate::{dep_manager::{Dependency, Configuration, Dependency_List}, version_mod::Version};
extern crate toml;

#[inline(always)]
#[allow(dead_code)]
pub fn profile_handler() -> &'static ProfileHandler {
    &ProfileHandler
}

// 作为一个工具类，用来处理配置文件
#[derive(Clone, Debug)]
pub struct ProfileHandler;

impl ProfileHandler{
    
    
    // 读取txt文件解析Template
    pub fn analyse (&self, path : String) -> Configuration{
        // 读取 TOML 文件内容
        let toml_content = fs::read_to_string(path).expect("Unable to read file");
        // 解析 TOML 格式数据
        let list: Dependency_List = toml::from_str(&toml_content).expect("Failed to parse TOML");
        println!("{:?}", list);
        // 初始化一个新的Configuratio
        let configuration = Configuration::new(list, "待实现".to_string(), Version::new("待实现".to_string()));
        // 完成一个程序的配置文件的解析，接下来交给后续管理器检查下载
        return configuration;
    }
}