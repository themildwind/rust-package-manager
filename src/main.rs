use simple_logger::{SimpleLogger};
use toml::to_string_pretty;
use crate::dep_manager::{Dependency, Dependency_List};
use crate::scheduler_module::scheduler;
use crate::version_mod::Version;
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
    scheduler().analyse_download_install("template.txt".to_string());
    log::info!("god bless me");
}
// fn main() {
//     // 创建一个 DependencyList 实例
//     let dependency_list = Dependency_List {
//         dependencies: vec![
//             Dependency {
//                 archive: "jammy-updates".to_string(),
//                 component: "main".to_string(),
//                 origin: "Ubuntu".to_string(),
//                 label: "Ubuntu".to_string(),
//                 architecture: "amd64".to_string(),
//                 download: "https://example.com/ubuntu-22.04.1-live-server-amd64.iso".to_string(),
//                 others: "其他".to_string(),
//                 version: Version { version: "22.04".to_string() },
//             },
//             // 添加更多的 Dependency 对象
//         ],
//     };

//     // 将 DependencyList 实例序列化为 TOML 格式的字符串
//     let toml_string = to_string_pretty(&dependency_list).unwrap();

//     // 打印生成的 TOML 字符串
//     println!("{}", toml_string);
// }

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

