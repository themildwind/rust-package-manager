use std::str::FromStr;
use std::{fs, sync::Arc};
use semver::Version;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use crate::entity::dependency::{self, BootstrapConfiguration, BootstrapConfigurationTemp, Dependency, Package, PackageList, PackageListTemp};
use crate::entity::software::{self, Software, SoftwareListTemp};
use crate::entity::version_wrapper::VersionWrapper;
use crate::error::software_error::SoftwareManagerError;
use crate::manager::package_manager::PackageManagerError;

extern crate toml;

#[inline(always)]
#[allow(dead_code)]
pub fn profile_handler() -> &'static ProfileHandler {
    &ProfileHandler
}
#[derive(Deserialize, Serialize, Debug, Clone)]
struct DependencyListTemp{
    dependencies : Vec<String>
}
// 作为一个工具类，用来处理配置文件
#[derive(Clone, Debug)]
pub struct ProfileHandler;

impl ProfileHandler{
    
    
    // 读取本地文件并解析
    pub fn get_local_file (&self, path : String) -> Result<String, SoftwareManagerError>{
        match fs::read_to_string(path) {
            Ok(c) => {
                println!("{}", c);
                return Ok(c)},
            Err(e) => return Err(SoftwareManagerError::ReadLocalOtherFileError(e.to_string())),
        };
    }
    // 解析string
    pub fn from_string_to_dependencies (&self, content : String) -> Result<Vec<Arc<Dependency>>, SoftwareManagerError>{
        if content.is_empty() {
            return Ok(Vec::new());
        }
        // 解析 TOML 格式数据
        let vec : DependencyListTemp = match toml::from_str(&content) {
            Ok(d) => d ,
            Err(e) => return Err(SoftwareManagerError::ParseDependencyError(e.to_string())),
        };
        
        let mut depends : Vec<Arc<Dependency>> = Vec::new();
        for dep_str in vec.dependencies {
            let parts : Vec<&str> = dep_str.split('-').collect();
            if parts.len() < 2 {
                return Err(SoftwareManagerError::ParseDependencyError(dep_str));
            }
            match Version::from_str(parts[1]) {
                Ok(version) => {
                    depends.push(Arc::new(Dependency::new(parts[0].to_string(), VersionWrapper::new(version))))
                },
                Err(e) => {
                    return Err(SoftwareManagerError::ParseDependencyError(e.to_string()));
                }
            }
        }
        return Ok(depends);
    }
    pub fn analyse_bootstrap_file (&self, path : String) -> Result<BootstrapConfiguration, SoftwareManagerError>{
        let toml_content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(SoftwareManagerError::ReadLocalOtherFileError(e.to_string())),
        };
        let temp : BootstrapConfigurationTemp = match toml::from_str(&toml_content) {
            Ok(list) => list,
            Err(e) => return Err(SoftwareManagerError::ParseDependencyError(e.to_string())),
        };
        match BootstrapConfiguration::from_temp(temp) {
            Ok(configuration) => return Ok(configuration),
            Err(e) => return Err(e),
        }
    }
    pub fn analyse_software_file (&self, path : String) -> Result<Vec<Arc<Software>>, SoftwareManagerError>{
        let toml_content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(SoftwareManagerError::ReadLocalOtherFileError(e.to_string())),
        };
        let softwares : SoftwareListTemp = match toml::from_str(&toml_content) {
            Ok(list) => list,
            Err(e) => return Err(SoftwareManagerError::ParseDependencyError(e.to_string())),
        };
        return softwares.to_softwares();
    }
    pub fn analyse_package_file (&self, path : String) -> Result<Vec<Arc<Package>>, PackageManagerError>{
        let toml_content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => return Err(PackageManagerError::ReadLocalPackageFileError(e.to_string())),
        };
        let packages : PackageListTemp = match toml::from_str(&toml_content) {
            Ok(list) => list,
            Err(e) => return Err(PackageManagerError::ReadLocalPackageFileError(e.to_string())),
        };
        return Ok(packages.packages.into_iter().map(Arc::new).collect());
    }
    //pub fn from_string_to_dependencies
}