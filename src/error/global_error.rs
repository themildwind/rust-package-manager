use std::{collections::LinkedList, sync::Arc};

use crate::entity::dependency::{Dependency, Package};
use crate::manager::package_manager::PackageManagerError;
use crate::error::software_error::SoftwareManagerError;

#[derive(Debug)]
pub enum GlobalError {
    // 循环依赖
    CircularDependency(LinkedList<Arc<Dependency>>),
    // 依赖不存在
    DependencyNotFound(Arc<Package>),
    // 依赖包已经下载
    DependencyAlreadyInstalled(Arc<Package>),
    // 依赖包未下载
    DependencyNotInstalled(Arc<Package>),
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
            SoftwareManagerError::SoftwareNotFound(d) => GlobalError::DependencyNotFound(d),
            SoftwareManagerError::DownloadError(d) => GlobalError::DownloadError(d),
            SoftwareManagerError::ParseDependencyError(d) => GlobalError::ParseDependencyError(d),
            SoftwareManagerError::SoftwareLockError(s) => GlobalError::SoftwareLockError(s),
            SoftwareManagerError::ReadLocalSoftwareFileError(_) => todo!(),
            SoftwareManagerError::PackageInstalled => todo!(),
            SoftwareManagerError::PackageNotFound(_) => todo!(),
            SoftwareManagerError::PackageLockFailed => todo!(),
            SoftwareManagerError::PackageInstallFailed => todo!(),
            SoftwareManagerError::PackageUninstallFailed => todo!(),
            SoftwareManagerError::ReadLocalPackageFileError(_) => todo!(),
            SoftwareManagerError::ReadLocalOtherFileError(_) => todo!(),
        }
    }
}
impl From<PackageManagerError> for GlobalError {
    fn from(error: PackageManagerError) -> Self {
        match error {
            PackageManagerError::PackageInstalled => todo!(),
            PackageManagerError::PackageNotFound(_) => todo!(),
            PackageManagerError::PackageLockFailed => todo!(),
            PackageManagerError::PackageInstallFailed => todo!(),
            PackageManagerError::PackageUninstallFailed => todo!(),
            PackageManagerError::ReadLocalPackageFileError(_) => todo!(),
        }
    }
}
