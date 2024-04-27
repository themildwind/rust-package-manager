
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
use crate::{entity::dependency::{Package, PackageList}, manager::software_manager::SoftwareManagerError};
use crate::entity::software::{Software};
use crate::entity::version_wrapper::VersionWrapper;

use super::run_profile::profile_handler;
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
    // todo 设置目标地址，下载器下载文件且解压
    pub fn download_software(&self, package: Arc<Package>, target_path: &str) -> Result<Arc<Software>, SoftwareManagerError> {
        // 创建一个新的 tokio 运行时环境
        let rt = Runtime::new().unwrap();    
        // 在异步上下文中执行异步函数并等待结果返回
        let result = rt.block_on(async {
            self.download_software_async(package, target_path).await
        });
        return result;
    }
    // 
    async fn download_software_async(&self, dependency: Arc<Package>, target_path: &str) -> Result<Arc<Software>, SoftwareManagerError> {
        // 找到下载地址
        let downloadsite = dependency.download();
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
        match decompress_unit().install(decoded_data, target_path) {
            Ok(o) => return Ok(Software::new(target_path.to_string(), dependency)),
            Err(err) => return Err(err),
        } 
    }
    // 获取配置文件
    pub fn get_dependency_list(&self, dependency: Arc<Package>) -> Result<PackageList, SoftwareManagerError> {
        // 创建一个新的 tokio 运行时环境
        let rt = Runtime::new().unwrap();    
        // 在异步上下文中执行异步函数并等待结果返回
        let result = rt.block_on(async {
            self.get_dependency_list_async(dependency).await
        });
        return result;
    }
    async fn get_dependency_list_async(&self, dependency: Arc<Package>) -> Result<PackageList, SoftwareManagerError> {
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
// 解压组件，仅仅解压，不进行安装
lazy_static! {
    static ref DECOMPRESS_UNIT: Arc<DecompressUnit> = Arc::new(DecompressUnit::new());
}
//
#[inline(always)]
#[allow(dead_code)]
pub fn decompress_unit() -> &'static Arc<DecompressUnit> {
    &DECOMPRESS_UNIT
}
// 安装模块
pub struct DecompressUnit {}
impl DecompressUnit {
    
    pub fn new() -> DecompressUnit {
        return DecompressUnit {};
    }
    pub fn install(&self, decoded_data : Vec<u8>, target_path: &str) -> Result<(), SoftwareManagerError>{
        let compressed_data: Vec<u8> = decoded_data;
        // 将数据写入文件
        let mut file = match File::create(target_path+"output.tar") {
            Ok(f) => f,
            Err(err) => return Err(SoftwareManagerError::InstallDependencyError(err.to_string())),
        };
        match file.write_all(&compressed_data) {
            Ok(r) => return Ok(()),
            Err(err) => return Err(SoftwareManagerError::InstallDependencyError(err.to_string())),
        };
    }
}
