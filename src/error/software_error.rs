use std::{collections::LinkedList, sync::Arc};

use crate::{entity::dependency::{Dependency, Package}, manager::package_manager::PackageManagerError};

pub enum SoftwareManagerError {
    // 检测到环形依赖
    CircularDependency(LinkedList<Arc<Dependency>>),
    // 依赖包未找到
    SoftwareNotFound(Arc<Package>),
    // 下载时遇到错误
    DownloadError(String),
    // 解析依赖时遇到错误
    ParseDependencyError(String),
    // 读取software本地文件错误
    ReadLocalSoftwareFileError(String),
    // 
    ReadLocalOtherFileError(String),
    // software加锁失败
    SoftwareLockError(String),
    // (下面的都是底层出现的错误)重复存在
    PackageInstalled,
    // 不存在
    PackageNotFound(String),
    // 加锁失败
    PackageLockFailed,
    // 安装失败
    PackageInstallFailed,
    // 卸载失败
    PackageUninstallFailed,
    // 读取package本地文件错误
    ReadLocalPackageFileError(String),
}
impl From<PackageManagerError> for SoftwareManagerError {
    fn from(err: PackageManagerError) -> Self {
        match err {
            PackageManagerError::PackageInstalled => SoftwareManagerError::PackageInstalled,
            PackageManagerError::PackageNotFound(s) => SoftwareManagerError::PackageNotFound(s),
            PackageManagerError::PackageLockFailed => SoftwareManagerError::PackageLockFailed,
            PackageManagerError::PackageInstallFailed => SoftwareManagerError::PackageInstallFailed,
            PackageManagerError::PackageUninstallFailed => SoftwareManagerError::PackageUninstallFailed,
            PackageManagerError::ReadLocalPackageFileError(s) => SoftwareManagerError::ReadLocalPackageFileError(s),
        }
    }
    
}
impl SoftwareManagerError {
    pub fn to_string(&self) -> String {
        match self {
            SoftwareManagerError::CircularDependency(s) => {
                let mut str = String::new();
                for i in s.iter() {
                    str.push_str(&i.to_string());
                }
                str
            }
            SoftwareManagerError::SoftwareNotFound(s) => {
                format!("software not found: {}", s.to_string())
            }
            SoftwareManagerError::DownloadError(s) => {
                format!("download error: {}", s)
            }
            SoftwareManagerError::ParseDependencyError(s) => {
                format!("parse dependency error: {}", s)
            }
            SoftwareManagerError::ReadLocalSoftwareFileError(s) => {
                format!("read local software file error: {}", s)
            }
            SoftwareManagerError::ReadLocalOtherFileError(s) => {
                format!("read local other file error: {}", s)
            }
            SoftwareManagerError::PackageInstalled => {
                format!("package installed")
            }
            SoftwareManagerError::PackageNotFound(s) => {
                format!("package not found: {}", s)
            }
            SoftwareManagerError::PackageLockFailed => {
                format!("package lock failed")
            }
            SoftwareManagerError::PackageInstallFailed => {
                format!("package install failed")
            }
            SoftwareManagerError::PackageUninstallFailed => {
                format!("package uninstall failed")
            }
            SoftwareManagerError::ReadLocalPackageFileError(s) => {
                format!("read local package file error: {}", s)
            }
            SoftwareManagerError::SoftwareLockError(s) => {
                format!("software lock error: {}", s)
            }
        }
    }
}