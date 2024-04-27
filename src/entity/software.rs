use serde_derive::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, MutexGuard};
use std::hash::{Hash, Hasher};
use crate::entity::version_wrapper::VersionWrapper;
use crate::entity::dependency::{PackageList,Package};

use super::dependency::Dependency;
#[derive(Debug)]
pub struct Software {
    inner: Mutex<InnerSoftware>,
    // 
    pub archive: String,
    pub version: VersionWrapper,
    pub dependencies: Vec<Dependency>
}
#[derive(Debug, Clone)]
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
    pub fn dependency(&self) -> Arc<Package> {
        return self.dependency.clone();
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
    pub fn new(archive : str, version : VersionWrapper) -> Arc<Software> {
        return Arc::new(Software {
            inner: Mutex::new(InnerSoftware {
                reference_count: 0,
                status: SoftwareStatus::Available, 
            }),
            archive : archive,
            version : version,
            dependencies: Vec::new(),
        });
    }

    pub fn inner(&self) -> Option<MutexGuard<InnerSoftware>>{
        self.inner.lock().ok()
    }
   
}