use std::{collections::{HashMap, HashSet, LinkedList}, sync::{Arc, Mutex}};
use lazy_static::lazy_static;
use serde::de;
use simple_logger::SimpleLogger;
use crate::dep_manager::{Dependency, Dependency_List};

// 安装的软件包
#[derive(Clone)]
pub struct Software
{
    // 指针，指向实际安装位置
    ptr : String,
    // 引用计数。 包括new version和 last version
    pub reference_count : i32,
    // 软件名称
    software_name : String,
    // 版本
    version : String,
}
impl Software {
    fn new (path : String, name : String, version : String) -> Software{
        return Software{
            ptr : path,
            reference_count : 0,
            software_name : name,
            version : version
        };
    }
    // 对软件包的引用计数做修改
}

// 管理器作为单例
lazy_static! {
    static ref SOFTWARE_MANAGER : Arc<Mutex<SoftwareManager>> = Arc::new(Mutex::new(SoftwareManager::new()));
}
#[inline(always)]
#[allow(dead_code)]
pub(super) fn software_manager() -> &'static Arc<Mutex<SoftwareManager>> {
    &SOFTWARE_MANAGER
}
// 有一个对软件包做管理的类，持有所有下载的软件包
// 垃圾回收，删除所有引用为0的软件包
pub struct SoftwareManager{
    softwares : Vec<Software>,
    hashmap : HashMap<Dependency, bool>,
}
impl SoftwareManager {
    fn new () -> SoftwareManager {
        return SoftwareManager{
            softwares : Vec::new(),
            hashmap : HashMap::new(),
        }
    }
    // 删除所有引用数为0的依赖包
    pub fn garbage_collection(&mut self){
        // 检查引用是否为0 ，为0则删除文件，再从数组移除
        let mut indexes_to_remove = Vec::new();
        // 使用 for 循环找到要删除的元素的索引
        for (index, soft) in self.softwares.iter().enumerate() {
            if  soft.reference_count == 0 {
                // todo 删除文件
                indexes_to_remove.push(index);
            }
        }
        // 根据索引删除元素
        for index in indexes_to_remove.iter().rev() {
            self.softwares.remove(*index);
        }
        log::info!("garbage collection success");
    }
    // 添加一个新软件
    pub fn insert(&mut self, software: Software, dependency : Dependency) {
        if !self.hashmap.contains_key(&dependency) {
            self.softwares.push(software);
            self.hashmap.insert(dependency, true);
        }
    }
    // 检查dependency_list是否存在环
    pub fn check(& self, dependency_list : Dependency_List) -> LinkedList<Dependency>{
        // 用于判断是否已经下载或遍历时已经加入下载队列
        let mut hashmap = self.hashmap.clone();
        // 用于记录下载队列，按顺序，无需其它依赖的排前面
        let mut download_list : LinkedList<Dependency> = LinkedList::new();
        let mut hashset : HashSet<Dependency> = HashSet::new();
        let mut linkedlist : LinkedList<Dependency> = LinkedList::new();
        for dependency in dependency_list.dependencies {
            if !Self::dfs(self, dependency, &mut hashset, &mut linkedlist, &mut hashmap, &mut download_list) {
                return download_list;
            }
        }
        return download_list;
    }
    
    // 深度优先搜索查找环
    fn dfs(& self, dep : Dependency,   hashset :  &mut HashSet<Dependency>,  linkedlist :  &mut LinkedList<Dependency>
    , hashmap :  &mut HashMap<Dependency,bool>, download_list :  &mut LinkedList<Dependency>) -> bool{
        //
        let dependency = &dep;
        if hashmap.contains_key(&dependency) {
            return true;
        }
        if !hashset.insert(dependency.clone()) {
            log::error!("\n******* 环形如下 *******");
            for dep_tmp in linkedlist.iter().rev() {
                println!("{} {} -> ", dep_tmp.archive, dep_tmp.version);
                if dep_tmp == dependency{
                    break;
                }
            }
            log::error!("******* 完毕 *******");
            return false;
        }
        linkedlist.push_back(dependency.clone());
        // todo 根据dep获得dep的依赖
        let config = Self::get_dep(dependency.clone());
        for dependency in config.dependencies {
            if ! self.dfs( dependency, hashset, linkedlist, hashmap, download_list) {
                return false;
            }
        }
        linkedlist.pop_back();
        hashset.remove(dependency);
        hashmap.insert(dependency.clone(), true);
        download_list.push_front(dependency.clone());
        return true;
    }

    // 根据下载队列来安装
    pub fn install (&mut self, download_list :  LinkedList<Dependency>) {
        // 将下载依赖交给下载器依次下载
        for dependency in download_list.iter() {
            let software = download_unit().download(dependency);
            // 加入新下载的依赖包
            self.insert(software, dependency.clone());
        }
    }

    // todo 查找依赖包的依赖
    pub fn get_dep(dep : Dependency) -> Dependency_List{
        return Dependency_List::new(Vec::new());
    }
}

// 
const DOWNLOAD_SITE: &str = "www.";

// 
lazy_static! {
    static ref DOWNLOAD_UNIT : Arc<DownloadUnit>= Arc::new(DownloadUnit::new());
}
//
#[inline(always)]
#[allow(dead_code)]
pub fn download_unit() -> &'static Arc<DownloadUnit> {
    &DOWNLOAD_UNIT
}
pub struct DownloadUnit {
    
}
impl DownloadUnit {
    pub fn download(& self, dependency : &Dependency) -> Software{
        // 找到下载地址
        let archive = &dependency.archive;
        let version = &dependency.version;
        let downloadsite = DOWNLOAD_SITE.to_string() + &archive + &version;
        // todo 下载到本地
        let path = "  ".to_string();
        return Software::new(path, archive.to_string(), version.to_string());
    }
    // 同步
    // fn download_sync (){
    //     let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;
    //     println!("body = {:?}", body);
    // }
    // 异步
    // async fn download_async (){
    //     let body = reqwest::get("https://www.rust-lang.org").await?.text().await?;
    //     println!("body = {:?}", body);
    // }
    pub fn new () -> DownloadUnit{
        return DownloadUnit {  };
    }
    
    
}