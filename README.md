// 12.3 进展
知道APT的前台原理，从/pool/main中获得软件包，用第一个字母作为索引。
// 12.7进展
理解dpkg，dpkg为APT的后端管理。dpkg软件包，有一个control文件，记录软件包的名称、版本等信息。
之后将文件夹打包成deb文件。
在etc/apt/sources.list 中保存源地址网站，从里面获取索引信息和软件包。
在var/lib/apt/lists 中保存对应网址获取的索引信息，如mirrors.tuna.tsinghua.edu.cn_ubuntu_dists_jammy-backports_multiverse_cnf_Commands-amd64
、mirrors.tuna.tsinghua.edu.cn_ubuntu_dists_jammy_multiverse_binary-amd64_Packages 。
对应网址https://mirrors.tuna.tsinghua.edu.cn/ubuntu/dists/jammy-updates/ 
科普博客 https://zhuanlan.zhihu.com/p/598001523
“/var/cache/apt/archives”目录是APT的本地缓存目录，用来保存最新下载的Deb软件包。
官方文档 https://www.debian.org/doc/manuals/debian-faq/pkgtools.zh-cn.html
考虑前端参考apt，从源获取索引和下载软件包，在后端参考nix管理软件包，解决依赖冲突问题。
// 12.25  12.26
调度器和错误报告和日志输出
// 2024.1.2
do : 返回结果result应用。
todo : configuration的创建、更新，software的引用计数的维护。
// 1.4
do : 配置文件的更新（1）
todo : 配置文件更新
// 1.5 
do : 配置文件的更新（2），引用计数的修改
todo : 配置文件的更新（3）
// 1.7
do : 配置文件的更新（3） ， 测试模块（1）
todo : 对于configuration类，既要实现eq、hash trait，还要使用内部锁，需要有一个age成员来标识不同的config，但是这个成员会被修改，用到锁，矛盾出现。
// 1.12
do : 测试模块（2）,全局错误（1）、规范化（1）
todo : 引用计数需要重新理解和设定、持久化、嵌入到DrangonOS
// 1.15
do : 引用计数
todo : 修改回收机制 （未完成）
// 3.02
do : 初步了解rust后端框架Actix
todo : 嵌入dragonOS（未完成）rust服务器后端（未完成）
// 3.04
do : 服务器预期功能 对用户-查询依赖包版本列表、下载依赖包文件。 对管理者-增删改查依赖包文件
    简单了解rust后端框架Actix和seaorm
todo :
// 3.05
do : 完成基本框架。使用指令创建数据库的表，并用seaorm的cli生成entity实例
todo : 去完成业务代码
// 3.06
do : service层，装载APP
todo : 管理员用户登录
// 3.07
do : 补充业务代码，尝试打通Http请求查询
todo : 管理员用户登录，业务代码
// 3.11
do : 打通Http请求查询,完善业务代码。对于版本号，数据库用三个列保存，rust中引入Version库
todo : 管理员用户登录，业务代码。去修改数据库
// 3.12
do : 
todo : 管理员用户登录，需要完善管理员操作的业务代码。去修改数据库，对于版本号，数据库用三个列保存，rust中引入Version库
// 3.14
do : 完成数据库迁移和数据库修改。
todo : 管理员用户登录，需要完善管理员操作的业务代码。rust中引入Version库。设置cargo clippy检查规则。测试代码
// 3.16
do : 完成数据库的增加、删除、查找功能，测试成功
todo : rust中引入Version库。设置cargo clippy检查规则。测试代码
// 3.16
do : 增删改查功能全部实现且测试成功
todo : 传输文件，压缩包，前端解包. 压缩包tar格式的读取、传输、解压
// 4.03
do : tar格式读取、传输完成
todo : 用户前端实现
// 4.06
do : 修改downloadunit
todo : 实现installunit，安装，错误类型需要细化修改
// 4.09
do : Base64 编码传输，
todo : 用户前端功能代码补充
// 4.09
do : 文件传输没有问题。需要补充代码，需要修正
todo : 用户前端功能代码补充