use semver::Version;
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};
use std::hash::{Hash, Hasher};
use crate::entity::version_wrapper::VersionWrapper;
use crate::entity::dependency::{PackageList,Package};
use crate::error::software_error::SoftwareManagerError;

use super::dependency::Dependency;
#[derive(Debug)]
pub struct Software {
    inner: Mutex<InnerSoftware>,
    // 
    pub archive: String,
    pub version: VersionWrapper,
    pub dependencies: Vec<Dependency>
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SoftwareStatus {
    // 软件包是可用的
    Available,
    // 软件包是可用的
    Unavailable,
    //
    Removed,
    
}
#[derive(Debug, Clone)]
pub struct InnerSoftware {
    // 引用计数。 包括new version和 last version
    reference_count: u32,
    // 
    status: SoftwareStatus,
}

impl InnerSoftware{
    pub fn reference_count(&self) -> u32 {
        return self.reference_count;
    }
    // 对软件包的引用计数做修改
    pub fn add(&mut self) {
        self.reference_count += 1;
    }
    pub fn remove(&mut self) {
        self.status = SoftwareStatus::Removed;
    }
    pub fn descrease(&mut self) -> u32{
        if self.reference_count > 0 {
            self.reference_count -= 1;
            return self.reference_count;
        }
        else {
            return 0;
        }
    }
}

impl Software {
    pub fn new(archive : String, version : VersionWrapper, deps : Vec<Dependency>, count :u32, status: SoftwareStatus) -> Arc<Software> {
        return Arc::new(Software {
            inner: Mutex::new(InnerSoftware {
                reference_count: count,
                status: status, 
            }),
            archive : archive,
            version : version,
            dependencies: deps,
        });
    }

    pub fn inner(&self) -> Option<MutexGuard<InnerSoftware>>{
        self.inner.lock().ok()
    }
   
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareTemp {
    pub archive: String,
    pub version: VersionWrapper,
    pub dependencies: Vec<String>,
    pub reference_count: u32, 
    pub status: SoftwareStatus,
}
impl SoftwareTemp {
    pub fn new(archive: String, version: VersionWrapper, dependencies: Vec<String>, reference_count: u32, status: SoftwareStatus) -> SoftwareTemp{
        return SoftwareTemp {
            archive,
            version,
            dependencies,
            reference_count,
            status,
        };
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoftwareListTemp {
    pub software_temp: Vec<SoftwareTemp>,
}
impl SoftwareListTemp {
    pub fn to_softwares(&self) -> Result<Vec<Arc<Software>>, SoftwareManagerError> {
        let mut softwares : Vec<Arc<Software>> = Vec::new();
        // todo !!
        // for tmp in self.software_temp.iter() {
        //     let mut depends : Vec<Dependency> = Vec::new();
        //     for dep_str in tmp.dependencies.iter() {
        //     let parts : Vec<&str> = dep_str.split('-').collect();
        //         if parts.len() < 2 {
        //             return Err(SoftwareManagerError::ParseDependencyError(dep_str.to_string()));
        //         }
        //         match Version::from_str(parts[1]) {
        //             Ok(version) => {
        //                 depends.push(Dependency::new(parts[0].to_string(), VersionWrapper::new(version)))
        //             },
        //             Err(e) => {
        //                 return Err(SoftwareManagerError::ParseDependencyError(e.to_string()));
        //             }
        //         }
        //     }
        //     softwares.push(Software::new(tmp.archive, tmp.version, depends, tmp.reference_count, tmp.status))
        // }
        return Ok(softwares);
    }
}