use simple_logger::{SimpleLogger};
use toml::to_string_pretty;
// 大致流程，每个应用程序有个按照规约的配置文件，读取文件，检查依赖，下载未拥有的依赖，
// 然后把地址给程序，用户选择依赖升级，保存版本链，并支持回退。
// 最后，删除不再使用的软件包。
mod error;
mod manager;
mod tool;
mod test;
mod entity;
mod scheduler_module;
use reqwest;

use crate::scheduler_module::scheduler;

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



