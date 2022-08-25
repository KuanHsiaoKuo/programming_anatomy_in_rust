# IPFS: 星际文件系统(InterPlanetary File System)

![ipfs](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/ipfs.jpeg)

<!--ts-->
* [IPFS: 星际文件系统(InterPlanetary File System)](#ipfs-星际文件系统interplanetary-file-system)
   * [IPFS介绍](#ipfs介绍)
      * [1 什么是IPFS](#1-什么是ipfs)
      * [2 为什么有IPFS](#2-为什么有ipfs)
         * [HTTP的中心化是低效的, 并且成本很高](#http的中心化是低效的-并且成本很高)
         * [Web文件经常被删除](#web文件经常被删除)
         * [中心化限制了web的成长](#中心化限制了web的成长)
         * [互联网应用高度依赖主干网](#互联网应用高度依赖主干网)
      * [3 IPFS的目标](#3-ipfs的目标)
      * [4 IPFS包含哪些内容](#4-ipfs包含哪些内容)
         * [IPFS是一个协议，类似http协议](#ipfs是一个协议类似http协议)
         * [IPFS是一个web协议](#ipfs是一个web协议)
         * [IPFS是模块化的协议](#ipfs是模块化的协议)
         * [IPFS是一个p2p系统](#ipfs是一个p2p系统)
         * [IPFS天生是一个CDN](#ipfs天生是一个cdn)
         * [IPFS拥有命名服务](#ipfs拥有命名服务)
   * [IPFS如何工作](#ipfs如何工作)
   * [IPFS如何解决中心化服务器缺点](#ipfs如何解决中心化服务器缺点)
      * [1 下载速度快, 不再依赖主干网, 中心化服务器](#1-下载速度快-不再依赖主干网-中心化服务器)
      * [2 存储空间变得非常便宜](#2-存储空间变得非常便宜)
      * [3 安全](#3-安全)
      * [4 开放](#4-开放)
   * [IPFS的用途](#ipfs的用途)
   * [IPFS的POW机制](#ipfs的pow机制)
   * [IPFS家族](#ipfs家族)
      * [Main Projects](#main-projects)
      * [Contributions](#contributions)
      * [Movements](#movements)
      * [IPFS 与 Filecoin](#ipfs-与-filecoin)
         * [IPFS：数据的分发和定位（数据传输协议，类似HTTP协议）](#ipfs数据的分发和定位数据传输协议类似http协议)
         * [Filecoin: 数据存储（类似一个云存储）](#filecoin-数据存储类似一个云存储)
         * [IPFS和Filecoin共同依赖libp2p项目。](#ipfs和filecoin共同依赖libp2p项目)
   * [使用IPFS的应用](#使用ipfs的应用)
   * [IPFS网络如何运行](#ipfs网络如何运行)
   * [IPFS: NAT traversal](#ipfs-nat-traversal)
   * [BitSwap](#bitswap)
   * [IPFS非rust实现版本](#ipfs非rust实现版本)
   * [参考资源](#参考资源)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Aug 25 14:20:31 UTC 2022 -->

<!--te-->

## IPFS介绍

### 1 什么是IPFS

星际文件系统(InterPlanetary File System). IPFS 是一个分布式的web, 点到点超媒体协议. 可以让我们的互联网速度更快, 更加安全, 并且更加开放. IPFS协议的目标是取代传统的互联网协议HTTP。

### 2 为什么有IPFS

众所周知, 互联网是建立在HTTP协议上的. HTTP协议是个伟大的发明, 让我们的互联网得以快速发展.但是互联网发展到了今天HTTP逐渐出来了不足.

#### HTTP的中心化是低效的, 并且成本很高

使用HTTP协议每次需要从中心化的服务器下载完整的文件(网页, 视频, 图片等), 速度慢, 效率低. 如果改用P2P的方式下载, 可以节省近60%的带宽. P2P将文件分割为小的块, 从多个服务器同时下载, 速度非常快.

#### Web文件经常被删除

回想一下是不是经常你收藏的某个页面, 在使用的时候浏览器返回404(无法找到页面), http的页面平均生存周期大约只有100天. Web文件经常被删除(由于存储成本太高), 无法永久保存. IPFS提供了文件的历史版本回溯功能(就像git版本控制工具一样), 可以很容易的查看文件的历史版本, 数据可以得到永久保存

#### 中心化限制了web的成长

我们的现有互联网是一个高度中心化的网络. 互联网是人类的伟大发明, 也是科技创新的加速器. 各种管制将对这互联网的功能造成威胁, 例如: 互联网封锁, 管制, 监控等等.这些都源于互联网的中心化.而分布式的IPFS可以克服这些web的缺点.

#### 互联网应用高度依赖主干网

主干网受制于诸多因素的影响, 战争, 自然灾害, 互联网管制, 中心化服务器宕机等等, 都可能是我们的互联网应用中断服务. IPFS可以是互联网应用极大的降低互联网应用对主干网的依赖.

### 3 IPFS的目标

IPFS不仅仅是为了加速web. 而是为了最终取代HTTP协议, 使互联网更加美好

### 4 IPFS包含哪些内容

#### IPFS是一个协议，类似http协议

- 定义了基于内容的寻址文件系统
- 内容分发
- 使用的技术分布式哈希、p2p传输、版本管理系统
- IPFS是一个文件系统

有文件夹和文件 可挂载文件系统

#### IPFS是一个web协议

- 可以像http那样查看互联网页面
- 未来浏览器可以直接支持 ipfs:/ 或者 fs:/ 协议

#### IPFS是模块化的协议

- 连接层：通过其他任何网络协议连接
- 路由层：寻找定位文件所在位置
- 数据块交换：采用BitTorrent技术

#### IPFS是一个p2p系统

- 世界范围内的p2p文件传输网络
- 分布式网络结构
- 没有单点失效问题

#### IPFS天生是一个CDN

- 文件添加到IPFS网络，将会在全世界进行CDN加速
- bittorrent的带宽管理

#### IPFS拥有命名服务

- IPNS：基于SFS（自认证系统）命名体系
- 可以和现有域名系统绑定

## IPFS如何工作

1. IPFS为每一个文件分配一个独一无二的哈希值(文件指纹: 根据文件的内容进行创建), 即使是两个文件内容只有1个比特的不相同, 其哈希值也是不相同的.所以IPFS是基于文件内容进行寻址, 而不像传统的HTTP协议一样基于域名寻址.
2. IPFS在整个网络范围内去掉重复的文件, 并且为文件建立版本管理, 也就是说每一个文件的变更历史都将被记录(这一点类似版本控制工具git, svn等), 可以很容易个回到文件的历史版本查看数据.
3. 当查询文件的时候, IPFS网络根据文件的哈希值(全网唯一)进行查找. 由于每个文件的哈希值全网唯一, 查询将很容易进行.
4. 如果仅仅使用哈希值来区分文件的话, 会给传播造成困难, 因为哈希值不容易记忆, 就像ip地址一样不容易记忆, 于是人类发明的域名. IPFS利用IPNS将哈希值映射为容易记的名字
5. 每个节点除了存储自己需要的数据, 还存储了一张哈希表, 用来记录文件存储所在的位置. 用来进行文件的查询下载.

## IPFS如何解决中心化服务器缺点

### 1 下载速度快, 不再依赖主干网, 中心化服务器

整个IPFS系统是一个分布式的文件存储系统, 那么在下载相关数据的时候, 将从多个节点同时下载, 相比于HTTP从中心服务器的下载速度要快很多, 大家都用过P2P下载(比如: 迅雷, BitTorrent), IPFS下载过程跟这个类似.

### 2 存储空间变得非常便宜

由于IPFS使用的是区块链技术, 利用 Filecoin(也就是挖矿)来激励矿工分享自己的硬盘, 并且IFPS从全网去掉了冗余存储(从整个网络空间考虑, 这将大大节省网络存储空间), 将来的IPFS存储将会变得非常便宜(与我们现在的云盘, 各种中心化的CND相比较).

### 3 安全

中心化服务器目前很难抵挡DDoS攻击, 当大量的访问请求从四面八方涌来, 中心化的服务器几乎会在一瞬间瘫痪, 巨大的访问量随时可能造成服务器宕机. IPFS天生就拥有抵挡这种攻击的能力. 因为所有的访问将会被分散到不同的节点. 甚至攻击者自己也是节点之一. 某种程度上讲, IPFS甚至能抵挡量子计算的攻击.

### 4 开放

众所周知, 比特币是一种去中心化, 匿名的数据货币, 这些特性使得比特币无法被管制, 交易无法篡改. IPFS同样, 由于是建立在去中心化的分布式网络上的, 所以IFPS很难被中心化管理, 限制. 互联网将更加开放.

## IPFS的用途

> IPFS主要解决现有中心化服务器中的数据存储问题。它能够极大的降低数据存储的成本,提升数据下载速度。

> 那么凡是需要优化数据存储的地方几乎都可以使用IPFS来提升效率

> 下面是一张保存在ipfs网络的图片：

![](https://ipfs.io/ipfs/QmdDTor6dWzknFJPJuhJgrUYqd56WkFXYAxyxpEY7kUrEb)

- 在 /ipfs 和 /ipns 下面挂载全球文件系统：就是说我们所有的文件都可以存到上面.
- 挂载个人同步的文件夹, 可以自动进行版本管理, 自动备份. 也就意味着未来我们将拥有无限空间的网盘, 不用担心数据丢失, 不用担心隐私泄露(非对称加密). 国外的dropbox, 跟IPFS云盘相比, 都将变得微不足道 。
- 作为加密文件和数据共享系统。IPFS天生视乎就具备这样的能力, 文件加密, 数据共享, 都是小菜一碟.
- 作为带版本控制的软件包管理系统.
- 作为虚拟机的根文件系统
- 作为利用管理程序, 把IPFS作为虚拟机的引导文件系统：在线操作系统
- 作为数据库：应用可以直接操作IPFS的Merkle DAG数据结构, 并且可以使用IPFS的版本控制, 缓存. 试想一下我们的数据库直接存在IPFS的文件系统是什么体验? 自动备份, 永不丢失, 安全加密, 无限空间, 高速连接。
- 作为加密通讯平台，谁都别想窃听消息通信
- 作为加密CDN, 作为web的CDN, CDN功能全包.
- 永久web, 不存在不能访问的链接, 跟 404 说 byebye.

## IPFS的POW机制

## IPFS家族

> [Work | Protocol Labs](https://protocol.ai/work/)

### Main Projects

![image-20220618215731222](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220618215731222.png)

### Contributions

![image-20220618220109640](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220618220109640.png)

### Movements

![image-20220618220532130](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220618220532130.png)

### IPFS 与 Filecoin

#### IPFS：数据的分发和定位（数据传输协议，类似HTTP协议）

- 传输：数据在节点之间进行传输
- 定位：寻址，发现数据的存储位置
- 存储：自己提供存储（可以保证存储的安全性），其它节点不保证数据存储的安全性
- 用户：下载数据免费，自己提供服务器，自己搭建节点
- 存储内容：只存储节点自己感兴趣的内容

#### Filecoin: 数据存储（类似一个云存储）

- 存储：付费存储，用户付费，矿工和Filecoin网络保证存储的安全性
- 下载：付费下载，用户付费，矿工负责发送数据
- 用户：不需要自己提供存储，也不需要自己提供节点
- 存储内容：收费存储一切

#### IPFS和Filecoin共同依赖libp2p项目。

> Filecoin是IPFS的激励层，二者互补形式一对协议。为我们的互联网提供了很好的基础设施。


了解上述的基本内容后：

- 如果开发者仅仅想要的是一个安全、快速的云存储，那么选择Filecoin即可。
- 如果开发者除了数据存储需求，还需要分发数据，那么选择IPFS即可。
- 如果开发者既有数据存储需求，又有数据的分发需求，那么可以单独选择IPFS，也可以IPFS+Filecoin一起。

> 注意：IPFS可以做Filecoin的事情，存储，而filecoin并不能做IPFS的事情，数据传输

## 使用IPFS的应用

- ~~[akasha](https://github.com/AKASHAorg/Community/releases)： 基于以太坊和IPFS的社交网络~~
- ~~Alexandria：去中心化的内容发布平台~~
- [Arbore](https://github.com/MichaelMure/Arbore)：朋友之间的文件共享系统--相信很快就可以抛弃某度的云盘了
- [dtube](https://github.com/dtube/dtube)：利用IPFS作为存储的视频分享网站
- [git-ipfs-rehost](https://github.com/whyrusleeping/git-ipfs-rehost)：可以把github上的项目存储到IPFS上
- [ipfs-search](https://github.com/ipfs-search/ipfs-search)：基于IFPS的搜索引擎
- [ipfs-share](https://github.com/ipfs-shipyard/ipfs-share-files)：基于IFPS的文件分享
- [ipfs.pics](http://ipfs.pics/)：基于IFPS的图片分享网站
- [Orbit](https://github.com/orbitdb/orbit)：基于IFPS的分布式聊天工具
- [Partyshare](https://github.com/BusterLabs/Partyshare)：一个简单的文件共享系统
- http://computes.io：基于IPFS的分布式计算机（这个牛，把世界上的计算资源收集起来，构建一个巨大的分布式计算机）
- [OpenBazaar](https://github.com/OpenBazaar?type=source)：openbazaar是一个去中心化的淘宝，口号是“买卖自由/Buy and Sell
  Freely”，问题是一旦用户停止运行软件，商店就下线了，借助于IPFS，openbazaar2.0 打造一个离线商店。
- Ubuntu：著名的linux发行版本Ubuntu正在计算把发行版本转移到IPFS上来，目前正在讨论方案。

> 更多可在这里查看：
[Awesome IPFS](https://ipfs.io/ipfs/QmUQs8yuYbQ7VbMpRoL6RFz4dMuK4RqCCmPmJc5fK7P4Yc/)

> 一些过期项目：
[IPFS Inactive repositories](https://github.com/ipfs-inactive?type=source)

## IPFS网络如何运行

## IPFS: NAT traversal

## BitSwap

## IPFS非rust实现版本

## 参考资源

- [IPFS指南 - 知乎](https://www.zhihu.com/column/ipfsguide)
- [rs-ipfs/rust-ipfs: The InterPlanetary File System (IPFS), implemented in Rust.](https://github.com/rs-ipfs/rust-ipfs)
- [Rust IPFS - Open Collective](https://opencollective.com/rs-ipfs)
- [IPFS网络是如何运行的(p2p网络) - 知乎](https://zhuanlan.zhihu.com/p/33170031)
- [IPFS: NAT traversal(NAT穿越) - 知乎](https://zhuanlan.zhihu.com/p/33057094)
- [IPFS: BitSwap协议(数据块交换) - 知乎](https://zhuanlan.zhihu.com/p/33148036)
- [IPFS非rust实现版本](https://zhuanlan.zhihu.com/p/34158682)
