use semver::{Error, Version};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};
use std::hash::{Hash, Hasher};
use crate::entity::version_wrapper::VersionWrapper;

#[derive(Clone, Debug, Deserialize,Serialize,PartialEq, Eq,Hash)]
pub struct Dependency {
    pub archive: String,
    pub version_wrapper: VersionWrapper,
}
impl Dependency {
    pub fn new(archive: String, version_wrapper: VersionWrapper) -> Dependency {
        Dependency {
            archive,
            version_wrapper,
        }
    }
    pub fn to_string(&self) -> String {
        return format!("{}-{}", self.archive, self.version_wrapper.to_string());
    }
}
#[derive(Clone, Debug, Deserialize,Serialize,PartialEq, Eq,Hash)]
pub struct BootstrapConfigurationTemp {
    depends : Vec<String>,
    recommends : Vec<String>
}
#[derive(Clone, Debug, Deserialize,Serialize,PartialEq, Eq,Hash)]
pub struct BootstrapConfiguration {
    depends : Vec<Dependency>,
    recommends : Vec<Dependency>
}
impl BootstrapConfiguration {
    pub fn from_temp(temp : BootstrapConfigurationTemp) -> Result<BootstrapConfiguration,Error> {
        let depends : Vec<Dependency> = Vec::new();
        let recommends : Vec<Dependency> = Vec::new();
        for dep in temp.depends {
            let parts : Vec<&str> = &dep.split("-").collect();
            if parts.len() < 2 {
                return Error;
            }
            match Version::from_str(parts[1]) {
                Ok(version) => {
                    depends.push(Dependency::new(parts[0].to_string(), VersionWrapper::new(version)))
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        for dep in temp.recommends {
            let parts : Vec<&str> = &dep.split("-").collect();
            if parts.len() < 2 {
                return Error;
            }
            match Version::from_str(parts[1]) {
                Ok(version) => {
                    recommends.push(Dependency::new(parts[0].to_string(), VersionWrapper::new(version)))
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
        return Ok(BootstrapConfiguration{
            depends : depends,
            recommends : recommends
        });
    }

    
}
#[derive(Clone, Debug, Deserialize,Serialize,PartialEq, Eq,Hash)]
pub struct Package
{
    pub archive: String,
    pub version_wrapper: VersionWrapper,
    pub component: String,
    pub origin: String,
    pub label: String,
    pub architecture: String,
    pub download : String,
    pub others: String,
    pub dependencies : Vec<Dependency>,
}
impl Package{
    fn new () -> Package{
        return Package{
            archive: String::new(),
            version_wrapper: VersionWrapper::new(Version::new(0, 0, 0)),
            component: String::new(),
            origin: String::new(),
            label: String::new(),
            architecture: String::new(),
            download : String::new(),
            others: String::new(),
            dependencies : Vec::new(),
        };
    }
    pub fn to_string(&self) -> String{
        return format!("{}-{}", self.archive, self.version_wrapper.to_string());
    }
    pub fn download(&self) -> String{
        return self.download.clone();
    }
}

// 表示一个程序的所依赖的软件包集合
// 每个应用程序所有，会根据配置文件构建
// 并且构建版本链
#[derive(Clone, Debug, PartialEq, Eq,Hash)]
pub struct PackageList{
    // 一个数组，表示所有依赖的包
    pub dependencies : Vec<Arc<Package>>,
}
impl PackageList{
    pub fn new(vec : Vec<Package>) -> PackageList{
        return  PackageList{
            dependencies : vec
        .into_iter()
        .map(|dep| Arc::new(dep))
        .collect()
        }
    }
    // 接受一个包含依赖的 Vec<Dependency> 创建新的配置
    pub fn with_dependencies(vec: Vec<Arc<Package>>) -> PackageList{
        PackageList{
            dependencies: vec,
        }
    }
}

#[derive(Debug)]
// 表示一个程序的配置文件。一对一
pub struct Configuration{
    
    // 标记这个配置文件
    pub archive: String,
    inner : Mutex<InnerConfiguration>
}
#[derive(Debug, Clone,PartialEq, Eq,Hash)]
pub struct InnerConfiguration {
    // 持有所有的版本
    pub vec : Vec<PackageList>,
    pub age: usize,
}
impl InnerConfiguration{
    pub fn age(&self) -> usize {
        return self.age;
    }
    pub fn vec(&self) -> Vec<PackageList> {
        return self.vec.clone();
    }
    // 
    fn add(&mut self) {
        self.age += 1;
    }
    pub fn update(&mut self, list: PackageList) {
        self.vec.push(list);
        self.add();
    }
}
// 实现 PartialEq 和 Eq trait
impl PartialEq for Configuration {
    fn eq(&self, other: &Self) -> bool {
        self.archive == other.archive
    }
}

impl Eq for Configuration {}
//
// 实现 Hash trait
impl Hash for Configuration {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // 将 key 和 value 组合起来计算哈希值
        self.archive.hash(state);
    }
}
impl  Configuration  {
    // 传入依赖列表
    pub fn new(list : PackageList, archive : String, age : usize) -> Arc<Configuration>{
        let mut vec : Vec<PackageList> = Vec::new();
        vec.push(list);
        return Arc::new(Configuration{
            archive : archive,
            inner : Mutex::new(InnerConfiguration{
                vec : vec,
                age : age,
            })
        });
    }

    pub fn inner(&self) -> Option<MutexGuard<InnerConfiguration>>{
        self.inner.lock().ok()
    }
}