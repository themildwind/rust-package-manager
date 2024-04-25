use std::{collections::LinkedList, sync::Arc};

use crate::{dep_manager::{DependencyItem, ConfigurationManagerError}, software_manager::SoftwareManagerError};
#[derive(Debug)]
pub enum GlobalError {
    // 循环依赖
    CircularDependency(LinkedList<Arc<DependencyItem>>),
    // 依赖不存在
    DependencyNotFound(Arc<DependencyItem>),
    // 依赖包已经下载
    DependencyAlreadyInstalled(Arc<DependencyItem>),
    // 依赖包未下载
    DependencyNotInstalled(Arc<DependencyItem>),
    // 下载时遇到错误
    DownloadError(String),
    // 安装时遇到错误
    InstallDependencyError(String),
    // 解析依赖时遇到错误
    ParseDependencyError(String),
    // 读取本地文件失败
    ReadLocalFileError(String),
    // 删除依赖时遇到错误
    RemoveDependencyError(String),
    // software加锁失败
    SoftwareLockError(String),
    //****************************************************/
    // Configuration重复存在
    DuplicateConfiguration,
    // 配置不存在
    ConfigurationNotFound(String),
    // 更新失败
    ConfigurationUpdateFailed,
    // 加锁失败
    ConfigurationLockFailed,
    //****************************************************/
    
}
impl From<SoftwareManagerError> for GlobalError {
    fn from(error: SoftwareManagerError) -> Self {
        match error {
            SoftwareManagerError::CircularDependency(l) => GlobalError::CircularDependency(l),
            SoftwareManagerError::DependencyNotFound(d) => GlobalError::DependencyNotFound(d),
            SoftwareManagerError::DependencyAlreadyInstalled(d) => GlobalError::DependencyAlreadyInstalled(d),
            SoftwareManagerError::DependencyNotInstalled(d) => GlobalError::DependencyNotFound(d),
            SoftwareManagerError::DownloadError(d) => GlobalError::DownloadError(d),
            SoftwareManagerError::InstallDependencyError(d) => GlobalError::InstallDependencyError(d),
            SoftwareManagerError::ParseDependencyError(d) => GlobalError::ParseDependencyError(d),
            SoftwareManagerError::RemoveDependencyError(s) => GlobalError::RemoveDependencyError(s),
            SoftwareManagerError::SoftwareLockError(s) => GlobalError::SoftwareLockError(s),
            SoftwareManagerError::ReadLocalFileError(s) => GlobalError::ReadLocalFileError(s),
            
        }
    }
}
impl From<ConfigurationManagerError> for GlobalError {
    fn from(error: ConfigurationManagerError) -> Self {
        match error {
            ConfigurationManagerError::ConfigurationLockFailed => GlobalError::ConfigurationLockFailed,
            ConfigurationManagerError::ConfigurationNotFound(s) => GlobalError::ConfigurationNotFound(s),
            ConfigurationManagerError::ConfigurationUpdateFailed => GlobalError::ConfigurationUpdateFailed,
            ConfigurationManagerError::DuplicateConfiguration => GlobalError::DuplicateConfiguration,
        }
    }
}
