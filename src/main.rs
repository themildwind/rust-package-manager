use simple_logger::{SimpleLogger};
use toml::to_string_pretty;
use crate::dep_manager::{Dependency, DependencyList};
use crate::run_profile::profile_handler;
use crate::scheduler_module::scheduler;
use crate::software_manager::download_unit;
use crate::version_mod::Version;
use crate::test_backend::TestBackend;
// 大致流程，每个应用程序有个按照规约的配置文件，读取文件，检查依赖，下载未拥有的依赖，
// 然后把地址给程序，用户选择依赖升级，保存版本链，并支持回退。
// 最后，删除不再使用的软件包。
mod version_mod ;
mod dep_manager;
mod run_profile;
mod software_manager;
mod scheduler_module;
mod system_error;
mod global_error;
mod test_backend;
use reqwest;

// fn main() {
//     // // 开启日志
//     // SimpleLogger::new().init().unwrap();
//     // // 成功案例
//     // let result1 = scheduler().analyse_download_install("success_template.txt".to_string());
//     // println!("{:?}", result1);
//     // // 失败案例
//     // let result2 = scheduler().analyse_download_install("failed_template.txt".to_string());
//     // println!("{:?}", result2);
//     // // 
//     // scheduler().garbage_collection();
//     let result = download_unit().download_sync("http://127.0.0.1:8080".to_string());
//     println!("{:?}", result);
//     log::info!("god bless me");
// }
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let archive = "example_archive".to_string();
    let version = "1.0".to_string();
    test_backend::TestBackend::test_get_file_url_by_archive_version(&archive, &version);
    Ok(())
}


