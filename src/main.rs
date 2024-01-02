use simple_logger::{SimpleLogger};

use crate::scheduler_module::scheduler;
// 大致流程，每个应用程序有个按照规约的配置文件，读取文件，检查依赖，下载未拥有的依赖，
// 然后把地址给程序，用户选择依赖升级，保存版本链，并支持回退。
// 最后，删除不再使用的软件包。
mod version_mod ;
mod dep_manager;
mod run_profile;
mod software_manager;
mod scheduler_module;
mod system_error;
use reqwest;

fn main() {
    SimpleLogger::new().init().unwrap();
    scheduler().garbage_collection();
    scheduler().analyse_download_install("/home/lzq/clone/learning/tmp/src/template.txt".to_string());

    log::info!("god bless me");
}


// #[tokio::main]
// async fn main() -> Result<(), reqwest::Error> {
//     // 发起 GET 请求
//     let response = reqwest::get("https://www.baidu.com")
//         .await?;

//     // 检查是否成功
//     if response.status().is_success() {
//         // 将响应文本打印到控制台
//         let body = response.text().await?;
//         println!("Response body:\n{}", body);
//     } else {
//         println!("Request failed with status: {:?}", response.status());
//     }

//     Ok(())
// }

