use simple_logger::{SimpleLogger};
use toml::to_string_pretty;
use crate::dep_manager::{DependencyItem, DependencyItemList};
use crate::run_profile::profile_handler;
use crate::scheduler_module::scheduler;
// 大致流程，每个应用程序有个按照规约的配置文件，读取文件，检查依赖，下载未拥有的依赖，
// 然后把地址给程序，用户选择依赖升级，保存版本链，并支持回退。
// 最后，删除不再使用的软件包。
mod dep_manager;
mod run_profile;
mod software_manager;
mod scheduler_module;
mod system_error;
mod global_error;
mod test_backend;
mod network_module;
mod version_wrapper;
use reqwest;

fn main() {
    // 开启日志
    SimpleLogger::new().init().unwrap();
    // 成功案例
    let result1 = scheduler().analyse_download_install("success_template.txt".to_string());
    println!("{:?}", result1);
    // 失败案例
    let result2 = scheduler().analyse_download_install("failed_template.txt".to_string());
    println!("{:?}", result2);
    // 
    scheduler().garbage_collection();
    
    log::info!("god bless me");
}



