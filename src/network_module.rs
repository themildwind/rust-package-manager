use crate::{dep_manager::{DependencyItem, DependencyItemList}, run_profile::{self, profile_handler}, software_manager::{Software, SoftwareManagerError}};
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
//下载组件
lazy_static! {
    static ref DOWNLOAD_UNIT: Arc<DownloadUnit> = Arc::new(DownloadUnit::new());
}
//
#[inline(always)]
#[allow(dead_code)]
pub fn download_unit() -> &'static Arc<DownloadUnit> {
    &DOWNLOAD_UNIT
}
pub struct DownloadUnit {}
impl DownloadUnit {
    pub fn download_software(&self, dependency: Arc<DependencyItem>) -> Result<Arc<Software>, SoftwareManagerError> {
        // 创建一个新的 tokio 运行时环境
        let rt = Runtime::new().unwrap();    
        // 在异步上下文中执行异步函数并等待结果返回
        let result = rt.block_on(async {
            self.download_software_async(dependency).await
        });
        return result;
    }
    // 
    async fn download_software_async(&self, dependency: Arc<DependencyItem>) -> Result<Arc<Software>, SoftwareManagerError> {
        // 找到下载地址
        let downloadsite = dependency.download();
        // todo 下载到本地，本地路径暂不确定
        let path = "  ".to_string();
        //
        let client = reqwest::Client::new();
        let response = match client.get(downloadsite).send().await{
            Ok(r) => r,
            Err(err) => return Err(SoftwareManagerError::DownloadError(err.to_string())),
        };
        let file : Value = match response.text().await {
            Ok(f) => serde_json::from_str(&f).unwrap(),
            Err(err) => return Err(SoftwareManagerError::DownloadError(err.to_string())),
        };
        // 根据返回结果操作
        if file.get("status_code").is_some() {
            return Err(SoftwareManagerError::DownloadError(file.get("message").unwrap().to_string()));
        }
        if file.get("data").is_none() {
            return Err(SoftwareManagerError::DownloadError("data is none".to_string()));
        }
        // 获得返回结果，如果没有问题就安装到本地
        let data  = file.get("data").unwrap().as_str().unwrap();
        let decoded_data: Vec<u8> = match decode(data) {
            Ok(decoded) => decoded,
            Err(_) => {
                return Err(SoftwareManagerError::DownloadError("Failed to decode Base64 string.".to_string()));
            }
        };
        match install_unit().install(decoded_data) {
            Ok(o) => return Ok(Software::new(path, dependency)),
            Err(err) => return Err(err),
        } 
    }
    // 获取配置文件
    pub fn get_dependency_list(&self, dependency: Arc<DependencyItem>) -> Result<DependencyItemList, SoftwareManagerError> {
        // 创建一个新的 tokio 运行时环境
        let rt = Runtime::new().unwrap();    
        // 在异步上下文中执行异步函数并等待结果返回
        let result = rt.block_on(async {
            self.get_dependency_list_async(dependency).await
        });
        return result;
    }
    async fn get_dependency_list_async(&self, dependency: Arc<DependencyItem>) -> Result<DependencyItemList, SoftwareManagerError> {
        let url = format!("http://127.0.0.1:8080/api/v1/softwares/configuration?archive={}&version={}", dependency.archive, dependency.version_wrapper.version.to_string());
        let client = reqwest::Client::new();
        let response = match client.get(url).send().await{
            Ok(r) => r,
            Err(err) => return Err(SoftwareManagerError::DownloadError(err.to_string())),
        };
        let file  = match response.text().await {
            Ok(f) => f,
            Err(err) => return Err(SoftwareManagerError::DownloadError(err.to_string())),
        };
        return profile_handler().analyse_string(file);
    }
    pub fn new() -> DownloadUnit {
        return DownloadUnit {};
    }
}
// 安装组件
lazy_static! {
    static ref INSTALL_UNIT: Arc<InstallUnit> = Arc::new(InstallUnit::new());
}
//
#[inline(always)]
#[allow(dead_code)]
pub fn install_unit() -> &'static Arc<InstallUnit> {
    &INSTALL_UNIT
}
// 安装模块
pub struct InstallUnit {}
impl InstallUnit {
    
    pub fn new() -> InstallUnit {
        return InstallUnit {};
    }
    pub fn install(&self, decoded_data : Vec<u8>) -> Result<(), SoftwareManagerError>{
        let compressed_data: Vec<u8> = decoded_data;
        // 将数据写入文件
        let mut file = match File::create("output.tar") {
            Ok(f) => f,
            Err(err) => return Err(SoftwareManagerError::InstallDependencyError(err.to_string())),
        };
        match file.write_all(&compressed_data) {
            Ok(r) => return Ok(()),
            Err(err) => return Err(SoftwareManagerError::InstallDependencyError(err.to_string())),
        };
    }
}
