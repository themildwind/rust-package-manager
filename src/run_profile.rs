use std::{fs, sync::Arc};
use serde_derive::{Deserialize, Serialize};

use crate::{dep_manager::{Configuration, DependencyItem, DependencyItemList}, software_manager::SoftwareManagerError};
extern crate toml;

#[inline(always)]
#[allow(dead_code)]
pub fn profile_handler() -> &'static ProfileHandler {
    &ProfileHandler
}
// 一个仅仅用于解析toml格式的类
#[derive(Clone, Debug,Deserialize,Serialize)]
pub struct DependencyListTomlHandler{
    pub dependencies : Vec<DependencyItem>,
}
// 作为一个工具类，用来处理配置文件
#[derive(Clone, Debug)]
pub struct ProfileHandler;

impl ProfileHandler{
    
    
    // 读取本地文件并解析
    pub fn analyse_local_file (&self, path : String) -> Result<DependencyItemList, SoftwareManagerError>{
        // 读取 TOML 文件内容
        let toml_content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(SoftwareManagerError::ReadLocalFileError(e.to_string())),
        };
        return self.analyse_string(toml_content);
    }
    // 解析string
    pub fn analyse_string (&self, content : String) -> Result<DependencyItemList, SoftwareManagerError>{
        if content.is_empty() {
            return Ok(DependencyItemList::new(vec![]));
        }
        // 解析 TOML 格式数据
        let vec: DependencyListTomlHandler = match toml::from_str(&content) {
            Ok(list) => list,
            Err(e) => return Err(SoftwareManagerError::ParseDependencyError(e.to_string())),
        };
        let dependency_list = DependencyItemList::new(vec.dependencies.clone());
        return Ok(dependency_list);
    }
}