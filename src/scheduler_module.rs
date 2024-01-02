use lazy_static::lazy_static;
use crate::{run_profile::profile_handler, software_manager::software_manager};

// 调度器作为单例
lazy_static! {
    static ref SCHEDULER : Scheduler = Scheduler::new();
}
#[inline(always)]
#[allow(dead_code)]
pub(super) fn scheduler() -> &'static Scheduler {
    &SCHEDULER
}
pub struct  Scheduler{

}
impl Scheduler {
    fn new ()-> Scheduler{
        return Scheduler {  };
    }
    // 调度器负责完成任务的调度，操控多个组件一起完成任务
    // todo 接受命令，解析某配置文件并下载依赖
    pub fn analyse_download_install (&self, path : String) {
        // 获取要下载的配置文件
        let config = profile_handler().analyse(path);
        // 检查
        let dependency_list = config.configs.last().unwrap().clone();
        let mut guard = software_manager().lock().unwrap();
        let download_list =  guard.check(dependency_list);
        // 下载
        guard.install(download_list);
    }
    // todo 更新某指定依赖
    // todo 垃圾回收
    pub fn garbage_collection(&self) {
        // 调用软件管理器
        let mut guard = software_manager().lock().unwrap();
        guard.garbage_collection();
    }
}