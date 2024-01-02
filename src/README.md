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
