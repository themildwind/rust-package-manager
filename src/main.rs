use std::{fs::File, io::Write, sync::Arc};

use entity::{dependency::{Dependency, Package}, software::{Software, SoftwareTemp}, version_wrapper::VersionWrapper};
use semver::Version;
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

use crate::{manager::{package_manager::package_manager, software_manager::software_manager}, scheduler_module::scheduler, tool::resolve_file::profile_handler};

fn main() {
    // 开启日志
    SimpleLogger::new().init().unwrap();
    //get_software_toml();
    //test_toml();
    log::info!("god bless me");
    //software_manager();
    //get_package_toml();
    package_manager();
}

fn test_toml() {
    let str = match profile_handler().get_local_file("database/output.toml".to_string()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("{}", e.to_string());
            return;
        }
    };
    let data = match profile_handler().from_string_to_dependencies(str) {
        Ok(s) => {
            for d in s {
                println!("{:?}", d);
            }
        },
        Err(e) => {
            log::error!("{}", e.to_string());
            return;
        }
    };
}
fn get_software_toml() {
    // 假设有一个对象需要输出为 TOML 格式
    let mut my_object : Vec<SoftwareTemp> = Vec::new();
    let mut vec : Vec<String> = Vec::new();
    let version = VersionWrapper::new(Version::new(1, 0, 1));
    vec.push(Dependency::new("testC".to_string(), version.clone()).to_string());
    my_object.push(SoftwareTemp::new("testB".to_string(), version.clone(), vec.clone(), 1, entity::software::SoftwareStatus::Available));
    vec.push(Dependency::new("testB".to_string(), version.clone()).to_string());
    my_object.push(SoftwareTemp::new("testA".to_string(), version.clone(), vec.clone(), 1, entity::software::SoftwareStatus::Available));
    // 将对象转换为 TOML 格式
    let toml_value = toml::to_string(&my_object).unwrap();
    // 将 TOML 格式字符串写入文件
    let mut file = File::create("database/software_data.toml").unwrap();
    file.write_all(toml_value.as_bytes()).unwrap();
}

fn get_package_toml() {
    // 假设有一个对象需要输出为 TOML 格式
    let mut my_object : Vec<Package> = Vec::new();
    let version = VersionWrapper::new(Version::new(1, 0, 1));
    my_object.push(Package::new("testA".to_string(), version.clone(), "component".to_string(), "origin".to_string(), "label".to_string(), "architecture".to_string(), "download".to_string(), "others".to_string()));
    my_object.push(Package::new("testB".to_string(), version.clone(), "component".to_string(), "origin".to_string(), "label".to_string(), "architecture".to_string(), "download".to_string(), "others".to_string()));
    
    // 将对象转换为 TOML 格式
    let toml_value = toml::to_string(&my_object).unwrap();
    // 将 TOML 格式字符串写入文件
    let mut file = File::create("database/package_data.toml").unwrap();
    file.write_all(toml_value.as_bytes()).unwrap();
}
