use std::{sync::Arc};
use lazy_static::lazy_static;
use crate::entity::dependency::{Configuration, Package, PackageList};
use crate::entity::software::{Software};
use crate::entity::version_wrapper::VersionWrapper;
use crate::error::global_error::GlobalError;
use crate::manager::package_manager::ConfigurationUpdateMode;
use crate::tool::resolve_file::profile_handler;
use crate::manager::software_manager::{self,software_manager};
use crate::error::software_error::SoftwareManagerError;

// 调度器作为单例
lazy_static! {
    static ref SCHEDULER : Scheduler = Scheduler::new();
}
#[inline(always)]
#[allow(dead_code)]
pub(super) fn scheduler() -> &'static Scheduler {
    &SCHEDULER
}
// 调度器负责完成任务的调度，操控多个组件一起完成任务
pub struct  Scheduler{
}
impl Scheduler {
    fn new ()-> Scheduler{
        return Scheduler {  };
    }
    // 解析下载安装
    // 使用场景： 1、 系统第一次启动时解析配置文件下载安装必带的依赖
    // pub fn analyse_download_install (&self, path : String) -> Result<(),GlobalError>{
    //     // 获取要下载的配置文件
    //     let dependency_list = match profile_handler().analyse_local_file(path.clone()) {
    //         Ok(list) => list,
    //         Err(e) => {
    //             return Err(GlobalError::from(e));
    //         }
    //     };
    //     // 检查
    //     let mut software_guard = crate::manager::software_manager::software_manager().lock().unwrap();
    //     let download_list = match software_guard.check(dependency_list.dependencies){
    //         Ok(list) => list,
    //         Err(e) => {
    //             log::error!("bug");
    //             // 存在环
    //             return Err(GlobalError::from(e));
    //         }     
    //     };
    //     for dep in download_list.iter(){
    //         print!(" {} ->",dep.archive);
    //     }
    //     //下载
    //     let download_result = software_guard.install(download_list);
    //     match download_result {
    //         Ok(_) => {},
    //         Err(e) => {
    //             return Err(GlobalError::from(e));
    //         }
    //     }
    //     // 把新的配置加入管理器
    //     let mut configuration_guard = configuration_manager().lock().unwrap();
    //     let config = Configuration::new(dependency_list.clone(), path, 0);
    //     match configuration_guard.insert(config) {
    //         Ok(_) => {},
    //         Err(e) => {
    //             log::error!("bug");
    //             return Err(GlobalError::from(e));
    //         }
    //     }
    //     // 对依赖的引用计数做修改
    //     match software_guard.update_reference(dependency_list) {
    //         Ok(_) => return Ok(()),
    //         Err(e) => return Err(GlobalError::from(e)) 
    //     }
    // }

    pub fn bootstrap (&self, path : String) -> Result<(),GlobalError>{
        // 获取要下载的配置文件
        let bootstrap_config = match profile_handler().analyse_bootstrap_file(path.clone()) {
            Ok(list) => list,
            Err(e) => {
                return Err(GlobalError::from(e));
            }
        };
        let mut software_manager_guard = software_manager().lock().unwrap();
        match software_manager_guard.install_package(bootstrap_config.get_depends()) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(GlobalError::from(e));
            }
        }
    }

    // // 
    // pub fn download_install (&self, dependency_list: PackageList) -> Result<(),GlobalError>{
    //     // 检查
    //     let mut software_guard = software_manager().lock().unwrap();
    //     let download_list ;
    //     match  software_guard.check(dependency_list.clone()){
    //         Ok(list) => download_list = list,
    //         Err(e) => {
    //             log::error!("bug");
    //             return Err(GlobalError::from(e));
    //         }
            
    //     }
    //     // 下载
    //     let download_result = software_guard.install(download_list);
    //     match download_result {
    //         Ok(_) => {},
    //         Err(e) => {
    //             return Err(GlobalError::from(e));
    //         }
    //     }
    //     // 对依赖的引用计数做修改
    //     match software_guard.update_reference(dependency_list) {
    //         Ok(_) => return Ok(()),
    //         Err(e) => return Err(GlobalError::from(e)) 
    //     }
    // }
    // // 更新某指定依赖
    // pub fn update_dependency(&self, archive : String, mode : ConfigurationUpdateMode) -> Result<(),GlobalError> {
    //     // 应用更新模式，让Manager进行更新，返回新的Configuration，然后对引用计数作修改
    //     let mut configuration_guard = configuration_manager().lock().unwrap();
    //     let new_list = match configuration_guard.update(archive, mode) {
    //         Ok(l) => l,
    //         Err(e) => {
    //             return Err(GlobalError::from(e));
    //         }
    //     };
    //     // 进行下载
    //     self.download_install(new_list);
    //     // todo 
    //     return Ok(());
    // }
    // // 垃圾回收
    // pub fn garbage_collection(&self) -> Result<(),GlobalError>{
    //     // 调用软件管理器
    //     let mut guard = software_manager().lock().unwrap();
    //     let result = guard.garbage_collection();
    //     // todo 向上输出全局类型的错误
    //     return Ok(());
    // }
    // pub fn bootstrap(&self) -> Result<(),GlobalError>{
    //     let path = "file/bootfile.txt".to_string();
    //     // 解析引导文件，获取必要安装软件
    //     let boot_config = match profile_handler().analyse_bootstrap_file(path) {
    //         Ok(c) => c,
    //         Err(e) => {
    //             return Err(GlobalError::from(e));
    //         }
    //     }; 
    //     // todo 调用上层管理器，由上层管理器获取更详细信息，然后调用底层管理器安装
    //     return Ok(());
    // }
    
}