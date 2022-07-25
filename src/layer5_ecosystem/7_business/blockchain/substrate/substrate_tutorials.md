# Substrate官方教程梳理与练习

<!--ts-->
* [Substrate官方教程梳理与练习](#substrate官方教程梳理与练习)
* [总览](#总览)
* [Get Started](#get-started)
   * [Build a local blockchain](#build-a-local-blockchain)
      * [设置开发环境](#设置开发环境)
         * [使用rustup设置rust环境](#使用rustup设置rust环境)
         * [检查环境](#检查环境)
      * [启动链节点](#启动链节点)
         * [下载node-template](#下载node-template)
         * [node-templeate项目结构](#node-templeate项目结构)
         * [Cargo.toml](#cargotoml)
         * [编译前的检查](#编译前的检查)
         * [编译](#编译)
         * [可能遇到的问题](#可能遇到的问题)
         * [本地运行节点](#本地运行节点)
         * [docker运行节点](#docker运行节点)
      * [前端访问](#前端访问)
         * [使用前端模版](#使用前端模版)
         * [使用polkadot-js访问节点](#使用polkadot-js访问节点)
      * [Substrate使用方式](#substrate使用方式)
         * [使用subtrate node](#使用subtrate-node)
         * [使用substrate frame](#使用substrate-frame)
         * [使用substrate core](#使用substrate-core)
   * [Simulate a network](#simulate-a-network)
   * [Add trusted nodes](#add-trusted-nodes)
      * [加密方式梳理](#加密方式梳理)
         * [Sr25519](#sr25519)
         * [Ed25519](#ed25519)
         * [SS58: 对应公钥](#ss58-对应公钥)
      * [步骤：](#步骤)
      * [actdiag](#actdiag)
   * [Authorize specific nodes](#authorize-specific-nodes)
      * [Alice授权Charlie过程](#alice授权charlie过程)
         * [Charlie连接Dave过程](#charlie连接dave过程)
      * [流程图](#流程图)
      * [总结](#总结)
   * [Monitor node metrics](#monitor-node-metrics)
      * [本节大概的架构](#本节大概的架构)
      * [安装Prometheus和grafana](#安装prometheus和grafana)
      * [配置Prometheus.yml](#配置prometheusyml)
      * [启动grafana](#启动grafana)
      * [配置数据源](#配置数据源)
      * [导入看板模版](#导入看板模版)
   * [Upgrade a running network](#upgrade-a-running-network)
      * [时序图](#时序图)
      * [第一次更新运行时](#第一次更新运行时)
      * [第二次上传文件设置自动执行条件](#第二次上传文件设置自动执行条件)
* [Work with pallets](#work-with-pallets)
   * [文档/代码更新问题](#文档代码更新问题)
   * [Pallet前置Rust知识](#pallet前置rust知识)
   * [Pallet组成](#pallet组成)
   * [Add a pallet to the runtime](#add-a-pallet-to-the-runtime)
      * [runtime结构分析](#runtime结构分析)
      * [runtime/Cargo.toml结构分析](#runtimecargotoml结构分析)
         * [[package]{...}](#package)
         * [[package.metadata.docs.rs]{...}](#packagemetadatadocsrs)
         * [[dependencies]{...}](#dependencies)
         * [[build-dependencies]{...}](#build-dependencies)
         * [[features]{...}](#features)
      * [四步添加pallet](#四步添加pallet)
         * [添加依赖: Cargo.toml/[dependincies]](#添加依赖-cargotomldependincies)
         * [添加feature: Cargo.toml/[features]](#添加feature-cargotomlfeatures)
         * [配置-&gt;添加config接口: src/lib.rs](#配置-添加config接口-srclibrs)
         * [定义运行时: src/lib.rs/construct_runtime!](#定义运行时-srclibrsconstruct_runtime)
      * [编译-&gt;运行-&gt;启动前端](#编译-运行-启动前端)
      * [验证功能](#验证功能)
         * [为帐户设置昵称](#为帐户设置昵称)
         * [使用Nicks pallet查询账户信息](#使用nicks-pallet查询账户信息)
      * [可能出现的问题](#可能出现的问题)
   * [Configure the contracts pallet](#configure-the-contracts-pallet)
      * [signed与sudo有不同权限。](#signed与sudo有不同权限)
   * [Use macros in a custom pallet](#use-macros-in-a-custom-pallet)
   * [Pallet组件深入](#pallet组件深入)
      * [1. Pallet Hooks](#1-pallet-hooks)
      * [2. Pallet Extrinsics](#2-pallet-extrinsics)
      * [3. Pallet Errors](#3-pallet-errors)
      * [4. Pallet Config](#4-pallet-config)
      * [5. Pallet Use Other Pallet](#5-pallet-use-other-pallet)
      * [6. Pallet Extension](#6-pallet-extension)
      * [7. Pallet Debug](#7-pallet-debug)
      * [8. Pallet RPC](#8-pallet-rpc)
      * [9. Pallet Benchmarking](#9-pallet-benchmarking)
* [Develop smart contracts](#develop-smart-contracts)
   * [Prepare your first contract](#prepare-your-first-contract)
   * [Develop a smart contract](#develop-a-smart-contract)
   * [Use maps for storing values](#use-maps-for-storing-values)
   * [Buid a token contract](#buid-a-token-contract)
   * [Troubleshoot smart contracts](#troubleshoot-smart-contracts)
* [Connect with other chains](#connect-with-other-chains)
   * [Start a local relay chain](#start-a-local-relay-chain)
   * [Connect a local parachain](#connect-a-local-parachain)
   * [Connect to Rococo testnet](#connect-to-rococo-testnet)
   * [Access EVM accounts](#access-evm-accounts)
* [参考资源](#参考资源)
   * [substrate文档练习](#substrate文档练习)
   * [pallet基础](#pallet基础)
      * [尝试添加pallet到runtime](#尝试添加pallet到runtime)
      * [编写pallet到rust前置知识](#编写pallet到rust前置知识)
      * [编写简单到pallet](#编写简单到pallet)
      * [pallet的组成](#pallet的组成)
   * [Pallet技巧细节](#pallet技巧细节)
      * [storage（链上）各个类型使用](#storage链上各个类型使用)
      * [Error类型的使用](#error类型的使用)
      * [写调度函数的套路](#写调度函数的套路)
      * [hooks的使用](#hooks的使用)
      * [pallet中的Config](#pallet中的config)
      * [在pallet中使用其它pallet](#在pallet中使用其它pallet)
      * [封装和扩展现有pallet](#封装和扩展现有pallet)
      * [调试](#调试)
      * [pallet中的类型转换；](#pallet中的类型转换)
      * [在pallet中使用链下工作者（Offchain worker）](#在pallet中使用链下工作者offchain-worker)
      * [在pallet中链上写本地存储（offchain index）；](#在pallet中链上写本地存储offchain-index)
      * [在pallet的ocw中使用链下存储（offchain storage）；](#在pallet的ocw中使用链下存储offchain-storage)
      * [在pallet中使用其它pallet（使用其它pallet的存储）；](#在pallet中使用其它pallet使用其它pallet的存储)
      * [在pallet中添加rpc接口](#在pallet中添加rpc接口)
      * [为某些trait提供默认实现。](#为某些trait提供默认实现)
   * [智能合约](#智能合约)
      * [初探ink!](#初探ink)
      * [深入ink!](#深入ink)
      * [ERC20](#erc20)
   * [连接其他链](#连接其他链)
      * [中继链连接](#中继链连接)
      * [平行链连接](#平行链连接)
   * [测试](#测试)
      * [编写测试](#编写测试)
      * [benchmarking](#benchmarking)
   * [升级](#升级)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Jul 25 04:20:06 UTC 2022 -->

<!--te-->

# 总览

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/substrate_tutorials.puml:1:}}
```

# Get Started

## Build a local blockchain

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/get-started/build-a-local-blockchain.puml:1:}}
```

> 这里主要使用官方提供的默认模版启动节点。

- [Build a local blockchain | Substrate_ Docs](https://docs.substrate.io/tutorials/get-started/build-local-blockchain/)

```admonish warn title='一定要注意文档是否更新'
由于rust对crate的版本只能检查，无法解决冲突问题，需要手动进行，所以一定要注意文档是否有更新，尤其是里面的代码
```

[New Substrate documentation released · Issue #1132 · substrate-developer-hub/substrate-docs](https://github.com/substrate-developer-hub/substrate-docs/issues/1132)
> 另外，substrate官方文档也一直处在更新状态中。

### 设置开发环境

#### 使用rustup设置rust环境

```shell
# 1.安装预编译包
sudo apt update && sudo apt install -y git clang curl libssl-dev llvm libudev-dev

# 2.安装Rust编译环境
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env
rustup default stable
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

#### 检查环境

```shell
rustc --version
rustup show
```

### 启动链节点

~~~admonish info title='node-template'
node-template实际上是官方提供的使用substrate开发的模板链，可以理解为substrate官方提供的样例，后续任何人想使用substrate可以在这个样例的基础上进行修改，这样开发链就更方便。
> 这就好比以前的好多山寨链，在btc的源码上改下创世区块的配置，就是一条新链。那么substrate其实也一样，提供了node-template这样一个模板，后续根据需求在这个上面改吧改吧，就能产生一条新链。
~~~

#### 下载node-template

```shell
git clone https://github.com/substrate-developer-hub/substrate-node-template
cd substrate-node-template
git checkout latest
```

~~~admonish warn title='⚠️注意查看最新分支的编号'
/home/substrate-node-template on  #polkadot-v0.9.24
~~~

#### node-templeate项目结构

~~~admonish info title='node-template项目结构'
```shell
tree -L 2                                                                                                                                                                                                                              ─╯
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── docker-compose.yml
├── docs
│   └── rust-setup.md
├── node
│   ├── Cargo.toml
│   ├── build.rs
│   └── src
├── pallets
│   └── template
├── runtime
│   ├── Cargo.toml
│   ├── build.rs
│   └── src
├── rustfmt.toml
├── scripts
│   ├── docker_run.sh
│   └── init.sh
├── shell.nix
└── target
    ├── CACHEDIR.TAG
    └── release

10 directories, 15 files
```
~~~

#### Cargo.toml

```toml
[workspace]
members = [
    "node",
    "pallets/template",
    "runtime",
]
[profile.release]
panic = "unwind"
```

> 可见node-template主要包含三部分：node、pallets/template、runtime

#### 编译前的检查

```shell
cargo check -p node-template-runtime
```

~~~admonish tip title='IDEA将会在修改Cargo.toml之后自动执行指令进行检查'
```shell
cargo metadata --verbose --format-version 1 --all-features
```
~~~

- [cargo metadata - The Cargo Book](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html)

> Output JSON to stdout containing information about the workspace members and resolved dependencies of the current
> package.

It is recommended to include the --format-version flag to future-proof your code to ensure the output is in the format
you are expecting.

#### 编译

```shell
cargo build --release
```

> 这个过程比较慢，会下载并编译上面三部分内的Cargo.toml列出的所有包

#### 可能遇到的问题

- 安装cmake

```shell
brew install cmake
```

#### 本地运行节点

```shell
./target/release/node-template --dev
```

#### docker运行节点

### 前端访问

[Build a local blockchain | Substrate_ Docs](https://docs.substrate.io/tutorials/get-started/build-local-blockchain/#install-the-front-end-template)

#### 使用前端模版

```shell
node --version
yarn --version
npm install -g yarn
git clone https://github.com/substrate-developer-hub/substrate-front-end-template
cd substrate-front-end-template
yarn install
yarn start
```

> 启动后访问本地：http://localhost:8000

#### 使用polkadot-js访问节点

```admonish info title='polkadot-js-app'
在substrate官方的教程中，是使用了substrate的前端模板来访问刚才启动的节点。但是在实际的开发中，后端人员其实更多的使用polkadot-js-app来访问我们的节点，所以这里我们也使用它来访问我们的节点。
```

- 在浏览器中输入https://polkadot.js.org/apps, 点击左上角会展开；

![CleanShot 2022-07-01 at 20.08.02@2x](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-01%20at%2020.08.02%402x.png)

- 在展开的菜单中点击DEVELOPMENT；

- 点击Local Node；

- 点击switch。

![CleanShot 2022-07-01 at 20.17.59@2x](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-01%20at%2020.17.59%402x.png)

### Substrate使用方式

> 使用substrate的方式主要有以下几种：

#### 使用subtrate node

开发者可以运行已经设计好的substrate节点，并配置genesis区块，在此方式下只需要提供一个json文件就可以启动自己的区块链。其实我们上一节的substrate初体验，也可以看成是使用此种方式的一个例子。

#### 使用substrate frame

frame其实是一组模块（pallet）和支持库。使用substrate frame可以轻松的创建自己的自定义运行时，因为frame是用来构建底层节点的。使用frame还可以配置数据类型，也可以从模块库中选择甚至是添加自己定义的模块。

#### 使用substrate core

使用substrate code运行开发者完全从头开始设计运行时（runtime，问题：什么是runtime？），当然此种方式也是使用substrate自由度最大的方式。

```admonish tip title='几种方式的关系可以用图描述如下：技术自由 vs 开发便利'
![Technical freedom vs development ease](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/technical-freedom.png)
```

## Simulate a network

> [返回顶部](#substrate官方教程梳理与练习)

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/get-started/simulate-a-network.puml:1:}}
```

## Add trusted nodes

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/get-started/add-trusted-nodes.puml:1:}}
```

### 加密方式梳理

- [substrate支持三种签名方案：sr25519、ed25519、secp256k1](https://docs.substrate.io/reference/command-line-tools/subkey/#signature-schemes)
- [Cryptography Explainer · Polkadot Wiki](https://wiki.polkadot.network/docs/learn-cryptography)
- [EdDSA - Wikipedia](https://en.wikipedia.org/wiki/EdDSA#Ed25519)
- [Secp256k1 - Bitcoin Wiki](https://en.bitcoin.it/wiki/Secp256k1)

#### Sr25519

> 用于使用 aura 为一个节点生成块。

- [substrate/sr25519.rs at master · paritytech/substrate](https://github.com/paritytech/substrate/blob/master/primitives/core/src/sr25519.rs)

#### Ed25519

> 用于使用 grapdpa 为一个节点生成块。

- [pub mod ed25519-substrate/lib.rs at 42b2d623d058197aebc3c737fb44fbbf278a85b4 · paritytech/substrate](https://github.com/paritytech/substrate/blob/42b2d623d058197aebc3c737fb44fbbf278a85b4/primitives/consensus/aura/src/lib.rs#L47-L63)

#### SS58: 对应公钥

- [pub use ss58_registry-substrate/crypto.rs at 0ba251c9388452c879bfcca425ada66f1f9bc802 · paritytech/substrate](https://github.com/paritytech/substrate/blob/0ba251c9388452c879bfcca425ada66f1f9bc802/primitives/core/src/crypto.rs#L46)

### 步骤：

1. 使用Sr25519 -> 一个助记词和对应SS58公钥 -> aura
2. 使用Ed25519+前面的助记词 -> Ed25519 公钥 -> grandpa

### actdiag

```kroki-actdiag
actdiag {
  first_sr25519 -> first_ed25519 -> add_validators
  second_sr25519 -> second_ed25519 -> add_validators
  export_customSpec -> add_validators
  add_validators -> first_start -> first_import_key -> first_restart
  add_validators -> second_start -> second_import_key -> second_restart
  first_restart -> peers
  second_restart -> peers

  lane node1 {
    label = "node1"
    first_sr25519 [label = "使用Sr25519 -> 一个助记词和对应SS58公钥 -> aura"];
    first_ed25519 [label = "使用Ed25519+前面的助记词 -> Ed25519 公钥 -> grandpa"];
    first_start [label = "使用链规范文件启动第一个节点"];
    first_import_key [label = "导入第一个节点的key"];
    first_restart [label = "使用链规范文件重启第一个节点"];
  }
  lane network {
    label = "本地链"
    export_customSpec [label = "导出链规范文件"];
    add_validators [label = "添加验证节点信息"];
    peers [label = "进入本地链，互为观察者(peers)"];
  }
  lane node2 {
    label = "node2"
    second_sr25519 [label = "使用Sr25519 -> 一个助记词和对应SS58公钥 -> aura"];
    second_ed25519 [label = "使用Ed25519+前面的助记词 -> Ed25519 公钥 -> grandpa"];
    second_start [label = "使用链规范文件启动第二个节点"];
    second_import_key [label = "导入第二个节点的key"];
    second_restart [label = "使用链规范文件重启第二个节点"];
  }
}
```

## Authorize specific nodes

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/get-started/authorize-specific-nodes.puml:1:}}
```

### Alice授权Charlie过程

1. 使用polkadot-js-app打开并切换到本地网络，开发者>超级管理(sudo)>nodeAuthorization

![image-20220723180351122](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220723180351122.png)

2. 切换到nodeAuthorization

![image-20220723180419349](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220723180419349.png)

3. 切换addConnections(node, owner)

![image-20220723180452122](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220723180452122.png)

4. 选择CHARLIE节点进行授权

![image-20220723180656562](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220723180656562.png)

- 注意填写charlie的peerid

  ![image-20220723182510593](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220723182510593.png)

5. 签名并提交

   ![image-20220723180850302](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220723180850302.png)

> 交易包含在区块中后，您应该看到 charlie 节点连接到 alice 和 bob 节点，并开始同步区块。这三个节点可以使用本地网络中默认启用的 mDNS 发现机制找到彼此。 如果您的节点不在同一个本地网络上，您应该使用命令行选项
> --no-mdns 来禁用它。

#### Charlie连接Dave过程

1. 切换Charlie账户，执行addConnections(node, connections)操作

![image-20220723183824019](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220723183824019.png)

> 注意：第一个填Chalie的peerid in hex，第二个填Dave的peer id in hex

2. 切换Dave账户，执行claimNode(node)操作

![image-20220723183609177](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220723183609177.png)

3. 提示，操作成功后右侧会出现弹窗

![image-20220723183551671](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220723183551671.png)

> 您现在应该看到 Dave 正在捕获区块，并且只有一个属于 Charlie 的节点！重新启动 Dave 的节点，以防它没有立即与 Charlie 连接

### 流程图

```kroki-mermaid
sequenceDiagram
    actor terminal as 终端
    participant runtime as 运行时:添加pallet
    participant node as 节点:修改链规范
    participant pkjs as polkadot-js-app
    terminal->>terminal: git chekout latest & cargo build --release
    terminal->>+runtime: 开始修改运行时cargo文件，添加pallet依赖与feature
    rect rgb(200, 150, 255)
    runtime->>runtime: runtime/Cargo.toml:depencies添加pallet-node-authorization
    runtime->>runtime: runtime/Cargo.toml:features添加pallet-node-authorization/std
    end
    runtime->>-terminal: prepare to check
    terminal->>terminal: cargo check -p node-template-runtime
    terminal->>+runtime: 开始给节点node添加pallet用到的参数类型、实现块、构建运行时配置
    rect rgb(200, 150, 255)
    runtime->>runtime: runtime/src/runtime.rs:add parameter_types
    runtime->>runtime: runtime/src/runtime.rs:add impl section
    runtime->>runtime: runtime/src/runtime.rs:add the pallet to the construct_runtime macro
    end
    runtime->>-terminal: 开始检查
    terminal->>terminal: cargo check -p node-template-runtime
    terminal->>+node: 开始给授权节点添加创始区块存储功能
    node->>node: node/Cargo.toml:add bs58 dependency
    rect rgb(200, 150, 255)
    node->>+node: 添加创始区块存储功能
    node->>node: node/src/node.rs:add genesis storage for nodes
    node->>node: node/src/node.rs:locate the testnet_genesis function
    node->>node: node/src/node.rs:add GenesisConfig declaration
    end
    node->>-terminal: cargo check & start nodes
    terminal->>terminal: cargo check -p node-template-runtime
    rect rgb(200, 150, 255)
    terminal->>terminal: start alice node
    terminal->>terminal: start bob node
    terminal->>terminal: start Charlie node
    terminal->>terminal: start Dave node
    end
    terminal->>pkjs: 开始进行授权与建立连接操作
    rect rgb(200, 150, 255)
    pkjs->>pkjs: 使用alice账号给Charlie授权
    pkjs->>pkjs: 使用Charlie账号连接Dave节点
    pkjs->>pkjs: Dave对外claimNode
    end
```

### 总结

任何节点都可以发出影响其他节点行为的交易(extrinsics)，只要它位于用于参考的链数据上，并且您在密钥库中拥有可用于所需来源的相关帐户的密钥。此演示中的所有节点都可以访问开发人员签名密钥，因此能够代表 Charlie
从网络上的任何连接节点发出影响 charlie 子节点的命令。

在现实世界的应用程序中，节点操作员只能访问他们的节点密钥，并且是唯一能够正确签署和提交外部信息的人，很可能来自他们自己的节点，他们可以控制密钥的安全性。

- [Accounts, addresses, and keys | Substrate_ Docs](https://docs.substrate.io/main-docs/fundamentals/accounts-addresses-keys/)

## Monitor node metrics

```admonish tip info title='承接关系:需要基于上一节课'
![image-20220724104945822](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220724104945822.png)
```

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/get-started/monitor-node-metrics.puml:1:}}
```

### 本节大概的架构

~~~admonish info title='prometheus和grafana配合流程'
```text
{{#include ../../../../../materials/svgbob/prometheus-grafana-seq.svgbob:1:}}
```
~~~

### 安装Prometheus和grafana

- [Download | Prometheus](https://prometheus.io/download/)
- [Download Grafana | Grafana Labs](https://grafana.com/grafana/download?edition=oss)

```shell
gunzip prometheus-<version>.darwin-amd64.tar.gz && tar -xvf prometheus-2.35.0.darwin-amd64.tar
brew update && brew install grafana
==> Downloading https://ghcr.io/v2/homebrew/core/grafana/manifests/9.0.2
######################################################################## 100.0%
==> Downloading https://ghcr.io/v2/homebrew/core/grafana/blobs/sha256:6022dd955d971d2d34d70f29e56335610108c84b75081020092e29f3ec641724
==> Downloading from https://pkg-containers.githubusercontent.com/ghcr1/blobs/sha256:6022dd955d971d2d34d70f29e56335610108c84b75081020092e29f3ec64
######################################################################## 100.0%
==> Pouring grafana--9.0.2.monterey.bottle.tar.gz
==> Caveats
To restart grafana after an upgrade:
  brew services restart grafana
Or, if you don't want/need a background service you can just run:
  /usr/local/opt/grafana/bin/grafana-server --config /usr/local/etc/grafana/grafana.ini --homepath /usr/local/opt/grafana/share/grafana --packaging=brew cfg:default.paths.logs=/usr/local/var/log/grafana cfg:default.paths.data=/usr/local/var/lib/grafana cfg:default.paths.plugins=/usr/local/var/lib/grafana/plugins
==> Summary
🍺  /usr/local/Cellar/grafana/9.0.2: 6,007 files, 247.3MB
==> Running `brew cleanup grafana`...
Disable this behaviour by setting HOMEBREW_NO_INSTALL_CLEANUP.
Hide these hints with HOMEBREW_NO_ENV_HINTS (see `man brew`).
```

### 配置Prometheus.yml

```yaml
# --snip--

# A scrape configuration containing exactly one endpoint to scrape:
# Here it's Prometheus itself.
scrape_configs:
  # The job name is added as a label `job=<job_name>` to any timeseries scraped from this config.
  - job_name: "substrate_node"

    # metrics_path defaults to '/metrics'
    # scheme defaults to 'http'.

    # Override the global default and scrape targets from this job every 5 seconds.
    # ** NOTE: you want to have this *LESS THAN* the block time in order to ensure
    # ** that you have a data point for every block!
    scrape_interval: 5s

    static_configs:
      - targets: ["localhost:9615"]
```

```shell
# specify a custom config file instead if you made one here:
./prometheus --config.file substrate_prometheus.yml
curl localhost:9615/metrics
```

```admonish info title='浏览器查看'
也可以直接打开浏览器：localhost:9615/metrics
```

### 启动grafana

```shell
# 后台运行
brew services restart grafana
# 指定运行
/usr/local/opt/grafana/bin/grafana-server --config /usr/local/etc/grafana/grafana.ini --homepath /usr/local/opt/grafana/share/grafana --packaging=brew cfg:default.paths.logs=/usr/local/var/log/grafana cfg:default.paths.data=/usr/local/var/lib/grafana cfg:default.paths.plugins=/usr/local/var/lib/grafana/plugins
```

- http://localhost:3000/

![image-20220724110946857](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220724110946857.png)

![image-20220724111058420](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220724111058420.png)

> 然后需要选择 Prometheus 数据源类型并指定 Grafana 需要查找它的位置。
>
> Grafana 需要的 Prometheus 端口不是在 prometheus.yml 文件 (http://localhost:9615) 中为节点发布其数据的位置设置的端口。
>
> 在同时运行 Substrate 节点和 Prometheus 的情况下，配置 Grafana 以在其默认端口 http://localhost:9090 或配置的端口（如果自定义它）上查找 Prometheus。

### 配置数据源

![CleanShot 2022-07-24 at 11.16.03](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-24%20at%2011.16.03.png)

![CleanShot 2022-07-24 at 11.16.59](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-24%20at%2011.16.59.png)

![CleanShot 2022-07-24 at 11.18.17](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-24%20at%2011.18.17.png)

![CleanShot 2022-07-24 at 11.34.47](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-24%20at%2011.34.47.png)

### 导入看板模版

[Export and import | Grafana documentation](https://grafana.com/docs/grafana/v7.5/dashboards/export-import/)

[Dashboards | Grafana Labs](https://grafana.com/grafana/dashboards/)

[Substrate Node Template Metrics dashboard for Grafana | Grafana Labs](https://grafana.com/grafana/dashboards/13759)

![image-20220724113036077](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220724113036077.png)

![CleanShot 2022-07-24 at 11.31.10](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-24%20at%2011.31.10.png)

![image-20220724113401390](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220724113401390.png)

## Upgrade a running network

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/get-started/upgrade-a-running-network.puml:1:}}
```

### 时序图

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/get-started/upgrade-a-running-network-seq.puml:1:}}
```

### 第一次更新运行时

1. 使用alice账户上传wasm文件
   ![CleanShot 2022-07-24 at 18.28.35](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-24%20at%2018.28.35.png)
2. node-template版本更新
   ![image-20220724183043916](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220724183043916.png)
3. 已经添加新的交易函数scheduler
   ![CleanShot 2022-07-24 at 18.41.44](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-24%20at%2018.41.44.png)

### 第二次上传文件设置自动执行条件

1. 使用scheduler函数
   ![CleanShot 2022-07-24 at 18.47.40](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-24%20at%2018.47.40.png)
2. 达到条件自动触发
   ![CleanShot 2022-07-24 at 18.49.14](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-24%20at%2018.49.14.png)

# Work with pallets

## 文档/代码更新问题

```admonisth warn title='substrate文档更新带来的问题'
由于目前substrate的源码和文档都在快速更新，所以可能出现一些未曾说过的问题。
比如链接找不到、目录里面不存在对应文章链接、编译时依赖包版本冲突。
这些都需要对文档的熟悉、对rust编程的熟悉才能轻松越过。
```

## Pallet前置Rust知识

{{#check Pallet-Preset| pallet 前置Rust知识}}

## Pallet组成

~~~admonish info title='pallet基础模版'
```rust
{{#include ../../../../../codes/substrate/pallet_components.rs:1:}}
```
~~~

```plantuml
{{#include ../../../../../materials/plantumls/pallet_components.mindmap:1:}}
```

## Add a pallet to the runtime

> 设置昵称：添加第一个Pallet到Runtime

> substrate node template提供了一个最小的可工作的运行时，但是为了保持精炼，它并不包括Frame中的大多数的Pallet

```admonish info title='官方教程地址'
[Add a pallet to the runtime | Substrate_ Docs](https://docs.substrate.io/tutorials/work-with-pallets/add-a-pallet/)
```

接下来接着使用前面的node template

### runtime结构分析

```shell
tree -L 2 runtime                                                                                               ─╯
runtime
├── Cargo.toml
├── build.rs
└── src
    └── lib.rs

1 directory, 3 files
```

### runtime/Cargo.toml结构分析

#### [package]{...}

#### [package.metadata.docs.rs]{...}

#### [dependencies]{...}

#### [build-dependencies]{...}

#### [features]{...}

### 四步添加pallet

#### 添加依赖: Cargo.toml/[dependincies]

```toml
pallet-nicks = { default-features = false, version = '4.0.0-dev', git = 'https://github.com/paritytech/substrate.git', tag = 'monthly-2021-08' }
```

~~~admonish warn title='排地雷'
由于官方文档和代码一直都在更新，可能会出现问题，这里就需要根据默认依赖的substrate分支进行更换
```toml
[dependencies]
sp-std = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.24" }
```
如上所示，对应的分支为：branch = "polkadot-v0.9.24", 所以需要改成：

```toml
[dependencies.pallet-nicks]
default-features = false
git = 'https://github.com/paritytech/substrate.git'
#tag = 'monthly-2021-10'
#tag = 'monthly-2022-04'
branch = "polkadot-v0.9.24"
version = '4.0.0-dev'
```
> 详见: [cargo 与 git](/layer2_design_abstract/6_module_manage/cargo_rustc.html#cargo-与-git-的关联)
~~~

#### 添加feature: Cargo.toml/[features]

```toml
[features]
default = ["std"]
std = [
    #--snip--
    'pallet-nicks/std',
    #--snip--
]
```

#### 配置->添加config接口: src/lib.rs

```rust
/// Add this code block to your template for Nicks:
parameter_types! {
    // Choose a fee that incentivizes desireable behavior.
    pub const NickReservationFee: u128 = 100;
    pub const MinNickLength: usize = 8;
    // Maximum bounds on storage are important to secure your chain.
    pub const MaxNickLength: usize = 32;
}

impl pallet_nicks::Config for Runtime {
    // The Balances pallet implements the ReservableCurrency trait.
    // https://substrate.dev/rustdocs/v3.0.0/pallet_balances/index.html#implementations-2
    type Currency = pallet_balances::Module<Runtime>;

    // Use the NickReservationFee from the parameter_types block.
    type ReservationFee = NickReservationFee;

    // No action is taken when deposits are forfeited.
    type Slashed = ();

    // Configure the FRAME System Root origin as the Nick pallet admin.
    // https://substrate.dev/rustdocs/v3.0.0/frame_system/enum.RawOrigin.html#variant.Root
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;

    // Use the MinNickLength from the parameter_types block.
    type MinLength = MinNickLength;

    // Use the MaxNickLength from the parameter_types block.
    type MaxLength = MaxNickLength;

    // The ubiquitous event type.
    type Event = Event;
}
```

#### 定义运行时: src/lib.rs/construct_runtime!

```rust
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        /* --snip-- */

        /*** Add This Line ***/
        Nicks: pallet_nicks::{Module, Call, Storage, Event<T>},
    }
);
```

### 编译->运行->启动前端

```shell
cargo build --release
./target/release/node-template --dev --tmp
yarn start
```

### 验证功能

#### 为帐户设置昵称

- 检查帐户选择列表以验证当前选择了 Alice 帐户。
- 在 Pallet Interactor 组件中，确认选择了 Extrinsic。
- 从可调用的托盘列表中选择nicks。
- 选择 **settName** 作为要从 nicks palette 调用的函数。
- 键入一个长于 MinNickLength（8 个字符）且不长于 MaxNickLength（32 个字符）的名称。
- 单击Signed以执行该功能。

![CleanShot 2022-07-03 at 10.51.26](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-03%20at%2010.51.26.png)

![CleanShot 2022-07-03 at 10.54.34](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-03%20at%2010.54.34.png)

#### 使用Nicks pallet查询账户信息

![CleanShot 2022-07-03 at 11.00.08](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/CleanShot%202022-07-03%20at%2011.00.08.png)

- 按图所示进行设置，查询，复制Alice的地址进行查询会返回一个元组，里面的两个值分别指：

    - Alice 帐户的十六进制编码昵称。
    - 为保护昵称而从 Alice 的账户中保留的金额。

> 如果使用Bob的地址，会返回None，因为没有给他设置昵称。

### 可能出现的问题

- [Conflicts when adding pallet to substrate-node-template · Issue #9 · substrate-developer-hub/pallet-did](https://github.com/substrate-developer-hub/pallet-did/issues/9)
- [substrate node template - "error: failed to select a version for `parity-util-mem`" - Substrate and Polkadot Stack Exchange](https://substrate.stackexchange.com/questions/2774/error-failed-to-select-a-version-for-parity-util-mem)

## Configure the contracts pallet

> [返回顶部](#substrate官方教程梳理与练习)

> 指定调用源头unsigned, signed or sudo

- [Specify the origin for a call | Substrate_ Docs](https://docs.substrate.io/tutorials/work-with-pallets/specify-origin/)

> 前面已经介绍用Alice的地址来设置并查询nickname(setName)，里面还有其他函数(killName、forceName、clearName)这里将会进行调用验证

### signed与sudo有不同权限。

点击Sudo按钮将会发出一个 Sudid 事件以通知节点参与者 Root 源发送了一个呼叫。 但是，内部调度会因 DispatchError 而失败（Sudo 按钮的 sudo 函数是“外部”调度）。

> 特别是，这是 DispatchError::Module 变体的一个实例，它会提供两个元数据：一个索引号和一个错误号。

- 索引号与产生错误的pallet有关；它对应于construct_runtime!中pallet的索引（位置）！。
- 错误编号与该托盘的错误枚举中相关变体的索引相对应。

> 使用这些数字查找托盘错误时，请记住索引是从零开始。

比如：

- 索引为 9（第十个托盘），对应nicks,
- 错误为 2（第三个错误）,
  对应[substrate源码](https://github.com/paritytech/substrate/blob/master/frame/nicks/src/lib.rs#L99-L108)中定义的第三个错误

```rust
/// Error for the nicks pallet.
#[pallet::error]
pub enum Error<T> {
    /// A name is too short.    
    TooShort,
    /// A name is too long.
    TooLong,
    /// An account isn't named.
    Unnamed,
}
```

- 取决于您的construct_runtime中尼克斯托盘的位置！宏，您可能会看到不同的索引编号。不管 index 的值如何，你应该看到错误值是 2，它对应于 Nick 的 Pallet 的 Error 枚举的第三个变体，Unnamed
  变体。这应该不足为奇，因为 Bob 尚未保留昵称，因此无法清除！

## Use macros in a custom pallet

> [返回顶部](#substrate官方教程梳理与练习)

## Pallet组件深入

> [返回顶部](#substrate官方教程梳理与练习)

### 1. Pallet Hooks

~~~admonish info title="基于执行过程看hooks"
```plantuml
{{#include ../../../../../materials/plantumls/substrate_activity_hooks.puml:1:}}
```
~~~

### 2. Pallet Extrinsics

{{#check Pallet-Extrinsics | pallet extrinsics 使用}}

### 3. Pallet Errors

{{#check Pallet-Errors | pallet errors 使用}}

### 4. Pallet Config

{{#check Pallet-Config | pallet config 使用}}

### 5. Pallet Use Other Pallet

{{#check Pallet-Use-Other-Pallet | pallet 使用其他 Pallet}}

### 6. Pallet Extension

{{#check Pallet-Extension | pallet 扩展 使用}}

### 7. Pallet Debug

{{#check Pallet-Debug | pallet 调试}}

### 8. Pallet RPC

{{#check Pallet-RPC | pallet rpc 使用}}

### 9. Pallet Benchmarking

{{#check Pallet-Benchmarking | pallet 基准测试}}

# Develop smart contracts

## Prepare your first contract

> [返回顶部](#substrate官方教程梳理与练习)

## Develop a smart contract

> [返回顶部](#substrate官方教程梳理与练习)

## Use maps for storing values

> [返回顶部](#substrate官方教程梳理与练习)

## Buid a token contract

> [返回顶部](#substrate官方教程梳理与练习)

## Troubleshoot smart contracts

> [返回顶部](#substrate官方教程梳理与练习)

# Connect with other chains

## Start a local relay chain

> [返回顶部](#substrate官方教程梳理与练习)

## Connect a local parachain

> [返回顶部](#substrate官方教程梳理与练习)

## Connect to Rococo testnet

> [返回顶部](#substrate官方教程梳理与练习)

## Access EVM accounts

> [返回顶部](#substrate官方教程梳理与练习)

# 参考资源

## substrate文档练习

- [构建一条链的体验](https://web.archive.org/web/20220628074841/https://mp.weixin.qq.com/s/2DuNX0-a14yPKT1nJNjk7Q)
  > substrate官方教程里面的[第一课](https://docs.substrate.io/tutorials/v3/create-your-first-substrate-chain/)名称叫做创建我们的第一条链，
  > 实际上我觉得应该叫做启动substrate默认模板链的节点更贴切，因为这个教程里面实际上就是把一个用substrate已经开发好的模板链的代码拉下来，然后编译一下，然后再启动起来。
  > 这个过程实际上和我们拉一个比特币的代码，然后编译下然后再启动 ，并没有太大的不同。

    - substrate 开发环境
      > - 启动链的节点：
          > 这里要用到node-template的代码。node-template实际上是官方提供的使用substrate开发的模板链，
          > 可以理解为substrate官方提供的样例，后续任何人想使用substrate可以在这个样例的基础上进行修改，这样开发链就更方便。
          > 这就好比以前的好多山寨链，在btc的源码上改下创世区块的配置，就是一条新链。 那么substrate其实也一样，提供了node-template这样一个模板，后续根据需求在这个上面改吧改吧，就能产生一条新链。
    - 使用polkadot-js访问节点:
      在substrate官方的教程中，是使用了substrate的前端模板来访问刚才启动的节点。但是在实际的开发中，后端人员其实更多的使用polkadot-js-app来访问我们的节点，所以这里我们也使用它来访问我们的节点。
    - [文档资料](https://docs.substrate.io/tutorials/v3/create-your-first-substrate-chain/)
- [Substrate快速入门](https://web.archive.org/web/20220628084629/https://mp.weixin.qq.com/s/GSWxgEA-CtdB8pi-gHe1ig)
  > substrate采用模块化的方法进行开发，它定义了一组丰富的原语，给开发人员提供了强大的、熟悉的编程方法。
    - 使用方式介绍：
        1. 使用substrate node: 使用json文件启动
        2. 使用substrate frame: 业务逻辑自由
           > frame其实是一组模块（pallet）和支持库。使用substrate
           frame可以轻松的创建自己的自定义运行时，因为frame是用来构建底层节点的。使用frame还可以配置数据类型，也可以从模块库中选择甚至是添加自己定义的模块。
        3. 使用substrate core: runtime自由
           > 使用substrate code运行开发者完全从头开始设计运行时（runtime，问题：什么是runtime？），当然此种方式也是使用substrate自由度最大的方式。
    - Substrate Client:
      > substrate客户端是基于substrate实现的区块链的节点客户端（可以理解为全节点），它主要由以下几个组件组成（以下也就是告诉我们实现一条链由哪几部分组成）：
        1. 存储: 用来维持区块链系统所呈现的状态演变。substrate提供了的存储方式是一种简单有效的key-value对存储机制的方式。
        2. Runtime: 这里就可以回答上面的问题，什么是runtime？runtime定义了区块的处理方式，主要是状态转换的逻辑。在substrate中，runtime code被编译成wasm作为区块链存储状态的一部分。
        3. p2p网络: 允许客户端和其它网络参与者进行通信。
        4. 共识: 提供了一种逻辑，能使网络参与者就区块链的状态达成一致。substrate支持提供自定义的共识引擎。
        5. RPC: 远程过程调用。
        6. telemetry: 通过嵌入式Prometheus服务器的方式对外展示（我理解应该是类似于区块链浏览器一样的东西，或者是提供信息给区块链浏览器展示）。

- [构建一个PoE(Prove of Existence)去中心化的应用](https://web.archive.org/web/20220628065030/https://mp.weixin.qq.com/s/MrnenO7AWhrf_-3Qs-aRJg)
  > substrat官方手册的第三个例子，使用substrate来创建自定义的存在证明dapp。我们本节的主要内容分为以下三步：
    1. 基于node template启动一条substrate的链。
    2. 修改node template来添加我们自己定制的Poe pallet，并实现我们的PoE API。
    3. 修改前端模板以添加与PoE API交互的自定义用户界面。

    - 接口设计
    - **创建自定义pallet**
      > node
      template的运行时是基于FRAME来实现的。FRAME是一个代码库，允许我们使用一系列pallet来构建底层的运行时。pallet可以看出是定义区块链功能的单个逻辑模块。subtrate为我们提供了一些已经构建好的pallet，我们在定义运行时时可以直接使用。
- [使用substrate构建私有网络](https://web.archive.org/web/20220628065034/https://mp.weixin.qq.com/s/w9Cj7jWkTo-PIBxtIyV9xQ)
  > 本节内容：
    1. 基于模板启动substrate区块链网络；
    2. 生成ed25519和sr25519 密钥对用于网络授权；
    3. 创建和编辑chainspec json文件。

    - [参考资料](https://substrate.dev/docs/en/tutorials/start-a-private-network/)
- [构建kitties链条](https://web.archive.org/web/20220628070053/https://mp.weixin.qq.com/s/1Yf6H6NoEOlpYQlBtuhy7A)
  > 分两部分，一是介绍如何构建kitties pallet，包括创建与kittes交互的功能；另一部分是介绍开发前端UI，与我们第一部分的链进行交互。
  > 目标：
    1. 学习构建和运行substrate节点的基本模式。
    2. 编写自定义框架pallet并集成到运行时。
    3. 了解如何创建和更新存储项。
    4. 编写pellet相关辅助函数。
    5. 使用PolkadotJs API将substrate节点连接到自定义前端。

    - kitties功能：
        1. 可以通过一些原始来源或者通过使用现有小猫进行繁殖创造。
        2. 以其所有者设定的价格出售。
        3. 从一个所有者转移到另一个所有者。
    - [参考资料](https://docs.substrate.io/tutorials/v3/kitties/pt1/#tutorial-objectives)
- [构建授权网络](https://web.archive.org/web/20220628070149/https://mp.weixin.qq.com/s/dOghynr1eVMdtXITwY5clQ)
  > 无许可准入区块链网络我们比较常见，例如比特币、以太坊都是无准入的。那么授权准入网络在哪些地方可能出现呢?
    1. 专用网络或者是联盟链网络；
    2. 高度管控的环境；
    3. 大规模测试预公开网络。
  > 目标
    1. 修改node-template工程添加node-authorization pallet。
    2. 加载部分节点并授权新节点加入网络。

## pallet基础

### 尝试添加pallet到runtime

- [添加一个pallet到runtime](https://web.archive.org/web/20220628065009/https://mp.weixin.qq.com/s/iQ6a-diWMfYDghuLVPJd9Q)
  > substrate node template提供了一个最小的可工作的运行时，但是为了保持精炼，它并不包括Frame中的大多数的Pallet。本节我们将学习如何将Pallet添加到runtime中。
    1. 安装Node Template
    2. 导入Pallet
    3. 配置Pallet
    4. 将Nicks添加到construct_runtime!中

    - [Add a pallet to the runtime | Substrate_ Docs](https://docs.substrate.io/tutorials/work-with-pallets/add-a-pallet/)

### 编写pallet到rust前置知识

- [learn-substrate-easy/5编写pallet的Rust前置知识.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/5%E7%BC%96%E5%86%99pallet%E7%9A%84Rust%E5%89%8D%E7%BD%AE%E7%9F%A5%E8%AF%86.md)
  > [Pallet前置知识](https://web.archive.org/web/20220627101518/https://mp.weixin.qq.com/s/wPVbEeIVKdXGro0QYsmJBg)
    - trait的孤儿规则
    - trait对象
    - trait的继承
    - 关联类型
    - 定义Config trait，然后为Pallet实现相应的trait，最后在main函数中使用它

### 编写简单到pallet

- [learn-substrate-easy/6编写简单的pallet.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/6%E7%BC%96%E5%86%99%E7%AE%80%E5%8D%95%E7%9A%84pallet.md)
    - node-template的结构
    - 编写pallet的一般格式, 整理出7个部分, 1和2基本上是固定的写法，而对于后面的3-7部分，则是根据实际需要写或者不写。关于模板中每部分的解释，可以参考文档.
        1. 依赖;
        2. pallet类型声明;
        3. config trait;
        4. 定义要使用的链上存储;
        5. 事件;
        6. 钩子函数;
        7. 交易调用函数;
    - 举例编写simple-pallet
      > 功能介绍: simple-pallet是一个存证的pallet，简单说就是提供一个存取一段hash到链上的功能，和从链上读取的功能。
    - 将pallet添加到runtime中
    - 编译运行
    - 调试使用pallet中的功能

```admonish info title='相关资料'
- [FRAME pallets | Substrate_ Docs](https://docs.substrate.io/reference/frame-pallets/#pallets) 
- [learn-substrate-easy-source/substrate-node-template/pallets/simple-pallet at main · KuanHsiaoKuo/learn-substrate-easy-source](https://github.com/KuanHsiaoKuo/learn-substrate-easy-source/tree/main/substrate-node-template/pallets/simple-pallet)
```

### pallet的组成

- [learn-substrate-easy/7Pallet的组成.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/7Pallet%E7%9A%84%E7%BB%84%E6%88%90.md)
  > 要想熟练的开发pallet，我们必须得把pallet中的各个组成部分弄清楚。本节，我们就按照模板中的各个部分的顺序来讲解pallet的组成

    1. 导出和依赖：Pub mod pallet{}就是将我们的pallet暴露出来， pub use pallet::*;是可以使用pallet中的所有类型，函数，数据等
    2. pallet类型声明：它是一系列trait和方法的拥有者，实际的作用类似于占位符，这里举例rust程序
    3. config trait： 这部分是指定Runtime的配置trait，Pallet中使用的一些类型和常量在此trait中进行配置。
    4. Storage-定义要使用的链上存储： 存储（Storage）允许我们在链上存储数据，使用它存储的数据可以通过Runtime进行访问。substrate提供了四种存储方式，分别是：
        - Storage Value: 存储单个的值, 无键
        - Storage Map: 以map方式存储，单键，key-value
        - Storage Double Map: 以双键方式存储，(key1, key2)-value
        - Storage N Map: 以多键方式存储，(key1, key2, ..., keyn)-value
    5. Event-事件：当pallet需要把运行时上的更改或变化通知给外部主体时，就需要用到事件。事件是一个枚举类型
    6. hooks-钩子函数：钩子函数，是在区块链运行过程中希望固定执行的函数，例如我们希望在每个区块构建之前、之后的时候执行某些逻辑等，就可以把这些逻辑放在钩子函数中
    7. Extrinsic-调度函数，交易调用函数: Extrinsic则是**可以从runtime外部可以调用的函数，也是pallet对外提供的逻辑功能**。比如交易
  > [路径用于引用模块树中的项 - Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#%E4%BD%BF%E7%94%A8-pub-%E5%85%B3%E9%94%AE%E5%AD%97%E6%9A%B4%E9%9C%B2%E8%B7%AF%E5%BE%84)

## Pallet技巧细节

### storage（链上）各个类型使用

- [learn-substrate-easy/8.1storage使用介绍.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.1storage%E4%BD%BF%E7%94%A8%E4%BB%8B%E7%BB%8D.md)
- [Runtime storage | Substrate_ Docs](https://docs.substrate.io/main-docs/build/runtime-storage/#best-practices)

```admonish tip title='区别pallet用到的storage和平时开发谈到的持久化storage'
在pallet中要使用的storage更多的其实是一个应用层的概念，如果用城市建造来类比，持久化存储就像是整个城市的马路或者是管道，而我们谈论的storage则是某个具体建筑或者房屋里面的水管会小路，至于这些小水管（或小路）是怎么和整个城市的大路联系起来的，不是我们讨论的范围。 
```

```rust
// 表示下面定义一个pallet storage
#[pallet::storage]
// 自动为storage生成一个getter函数，名字叫some_value
// 后续可以在pallet使用some_value()函数来获取该Storage中存储的值
#[pallet::getter(fn some_value)]
pub(super) type SomeValue = StorageValue<_, u64, ValueQuery>;
```

### Error类型的使用

- [pallet中Error类型的使用](https://web.archive.org/web/20220627112629/https://mp.weixin.qq.com/s/cNijF5h2Yn7R-K0ryoOJrA)
  >
  在runtime代码执行时，代码必须是“非抛出的”，或者说不应该panic，应该是优雅的处理错误，所以在写pallet代码时，允许我们自定义错误类型，当错误发生时，可以返回我们定义的错误类型。这里的Error类型是指运行时在执行调度函数（也就是交易函数）时返回的错误。因为在调度函数执行时，返回的结果为DispatchResult类型，当执行结果错误时，返回DispatchError。
    - 错误类型的定义
    - 在函数中返回错误
    - 简单示例

### 写调度函数的套路

- [substrate轻松学：写调度函数](https://mp.weixin.qq.com/s/Xnv5aNiLn-NoH6obouaONg)
  > 调度函数在substrate官方文档里面叫做Extrinsics（外部调用），详细的Extrinsics介绍可以参考这里.
  > 在substrate中共有三种Extrinsics，分别是Inherents、Signed transactions和Unsigned transactions。
  > 而在我们开发pallet的过程中，比较常用到的是后两种，所以我们这里也主要介绍后两种，对于Inherents有兴趣的小伙伴可以自己看官方文档研究下。
    - Signed transactions
    - Unsigned transactions
    - 通常写法：调度函数的位置->函数体的写法->权重->transactional
    - 示例
    -
  参考：[extrinsics](https://docs.substrate.io/v3/concepts/extrinsics/)
  &[weights-and-fees](https://docs.substrate.io/v3/runtime/weights-and-fees/)

### hooks的使用

- [learn-substrate-easy/8.4Hooks函数使用.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.4Hooks%E5%87%BD%E6%95%B0%E4%BD%BF%E7%94%A8.md)
- [hooks: pallet的🪝钩子函数使用](https://web.archive.org/web/20220628021501/https://mp.weixin.qq.com/s/tPyB9CuTVP2Y1DGgl_VPyQ)

  ![运行图](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/%E8%BF%90%E8%A1%8C%E5%9B%BE.png)

```admonish tip title='交易到打包的过程'
1. 用户通过钱包发起交易;
2. 和钱包相连的全节点收到交易后会把交易广播到网络中;
3. 然后根据共识算法打包区块，某个全节点获得了打包权（图中画的是节点4），
  然后将交易打包到区块中;
4. 打包好区块后，将区块广播到网络中;
5. 其它每个节点收到区块后验证，然后执行区块里面的交易，更新自己本地的账本。
```

    - substrate中的执行过程
        1. 初始化区块（Initializes the block）
        2. 执行区块（Executes extrinsics）
        3. 确认区块（ Finalizes the block）.
    - hooks介绍:
        1. on_finalize: 在区块 finalize 的时候调用。
        2. on_idle：区块finalize的时候调用，不过比on_finalize先调用。
        3. on_initialize：区块初始化的时候调用。
        4. on_runtime_upgrade：执行模块升级的时候调用。
        5. pre_upgrade：升级之前的检查。
        6. post_upgrade：升级之后的处理。
        7. offchain_worker：在一个 pallet 上实现此函数后可以在此函数中长时间的执行需要链下执行的功能。该函数会在每次区块导入的时候调用。后续我们讲ocw使用的时候就需要和这个函数打交道。
        8. integrity_test：运行集成测试。
    - 示例
    - [资料](https://docs.substrate.io/v3/concepts/execution/)
    - [substrate源码](https://paritytech.github.io/substrate/master/frame_support/traits/trait.Hooks.html)

### pallet中的Config

- [learn-substrate-easy/8.5pallet中的Config.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.5pallet%E4%B8%AD%E7%9A%84Config.md)
- [pallet中的config](https://web.archive.org/web/20220627112755/https://mp.weixin.qq.com/s/JOaBn4bkda2LicV3Lyb4tw)
    - 好好理解rust中关于trait和关联类型相关的知识
    - pallet 简单示例: 介绍一个存储学生信息的pallet，其中存储逻辑写在extrinsic中
    - 在Config中定义配置类型：主要使用trait约束和关联类型改写
    - 在runtime中指定具体的类型
    - 构建、交互与调试
    - [参考资料](https://docs.substrate.io/v3/runtime/events-and-errors/)

### 在pallet中使用其它pallet

- [learn-substrate-easy/8.6在pallet中使用其它pallet.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.6%E5%9C%A8pallet%E4%B8%AD%E4%BD%BF%E7%94%A8%E5%85%B6%E5%AE%83pallet.md)
- [在pallet中使用其他pallet](https://web.archive.org/web/20220627101725/https://mp.weixin.qq.com/s/z4fefNUb3avcae0htHpxgQ)
    - 在自己的pallet中使用其它的pallet主要有以下几种情况：
        1. 指定某个现成的pallet: 在pallet的config中定义类型，然后runtime中使用时指定这个类型为frame中指定某个现成的pallet；
        2. 指定某个自定义的pallet: 在pallet的config中定义类型，然后runtime中使用时指定这个类型为frame中指定某个自定义的pallet；
        3. 封装和扩展现有的 pallet 。
    - 在runtime中直接指定某个类型为其它的pallet
      > 这种方式比较常见的就是在pallet中定义currency类型，然后用指定currency类型为balances
      pallet。详细的可以看substrate中node中的使用，在pallet_assets中使用了pallet_balances，就是通过指定前者的currency类型为后者
        - [详情](https://github.com/paritytech/substrate/blob/master/bin/node/runtime/src/lib.rs#L1343)
    - pallet中使用其它pallet的storage
      > 自定义两个pallet，分别叫做pallet-use-other-pallet1和pallet-storage-provider，然后我们在前一个pallet中读取和存储后一个pallet

### 封装和扩展现有pallet

- [learn-substrate-easy/8.7封装和扩展现有pallet.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.7%E5%B0%81%E8%A3%85%E5%92%8C%E6%89%A9%E5%B1%95%E7%8E%B0%E6%9C%89pallet.md)
- [封装和扩展现有的pallet](https://web.archive.org/web/20220627113013/https://mp.weixin.qq.com/s/23wuRo4gj4oH-3EG74NnTA)
    - 这里使用substrate提供的contracts pallet，然后对其中的功能进行封装。
      > 在我们的封装中，将contracts pallet的call函数封装成sudo_call，即需要root权限才能调用。同时，我们在runtime中加载contracts时，去掉直接调用contracts函数的方式。
    - 整个方式我们分成两大步骤，如下：
        1. 编写extend-pallet;
        2. 在runtime配置extend- pallet 和contracts pallet。
    - [文档资料](https://www.shawntabrizi.com/substrate/extending-substrate-runtime-modules/)

### 调试

- [learn-substrate-easy/8.8调试.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.8%E8%B0%83%E8%AF%95.md)
- [调试pallet](https://web.archive.org/web/20220627113043/https://mp.weixin.qq.com/s/Ddu-CPgRz-U7uO4PkUnp2g)
    - 在pallet开发时主要有以下几种调试方式：
        1. logging uilities;
        2. printable trait;
        3. print函数;
        4. if_std.
    - 使用logging uilities
    - 使用 printable trait
    - 使用print函数
    - 使用 if_std
    - [文档资料](https://docs.substrate.io/v3/runtime/debugging/)

### pallet中的类型转换；

### 在pallet中使用链下工作者（Offchain worker）

- [learn-substrate-easy/8.9使用OCW提交签名交易.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.9%E4%BD%BF%E7%94%A8OCW%E6%8F%90%E4%BA%A4%E7%AD%BE%E5%90%8D%E4%BA%A4%E6%98%93.md)
- [learn-substrate-easy/8.10在ocw中提交未签名交易.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.10%E5%9C%A8ocw%E4%B8%AD%E6%8F%90%E4%BA%A4%E6%9C%AA%E7%AD%BE%E5%90%8D%E4%BA%A4%E6%98%93.md)
- [learn-substrate-easy/8.11在ocw中提交具有签名payload的未签名交易.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.11%E5%9C%A8ocw%E4%B8%AD%E6%8F%90%E4%BA%A4%E5%85%B7%E6%9C%89%E7%AD%BE%E5%90%8Dpayload%E7%9A%84%E6%9C%AA%E7%AD%BE%E5%90%8D%E4%BA%A4%E6%98%93.md)
- [learn-substrate-easy/8.12在ocw中使用链下存储.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.12%E5%9C%A8ocw%E4%B8%AD%E4%BD%BF%E7%94%A8%E9%93%BE%E4%B8%8B%E5%AD%98%E5%82%A8.md)
- [learn-substrate-easy/8.14在ocw中发送http请求.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.14%E5%9C%A8ocw%E4%B8%AD%E5%8F%91%E9%80%81http%E8%AF%B7%E6%B1%82.md)

### 在pallet中链上写本地存储（offchain index）；

- [learn-substrate-easy/8.13使用offchain_index.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.13%E4%BD%BF%E7%94%A8offchain_index.md)

### 在pallet的ocw中使用链下存储（offchain storage）；

### 在pallet中使用其它pallet（使用其它pallet的存储）；

- [learn-substrate-easy/8.6在pallet中使用其它pallet.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.6%E5%9C%A8pallet%E4%B8%AD%E4%BD%BF%E7%94%A8%E5%85%B6%E5%AE%83pallet.md)

### 在pallet中添加rpc接口

- [learn-substrate-easy/8.15在pallet中添加rpc接口.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/8.15%E5%9C%A8pallet%E4%B8%AD%E6%B7%BB%E5%8A%A0rpc%E6%8E%A5%E5%8F%A3.md)
- [为pallet自定义rpc接口](https://web.archive.org/web/20220627101825/https://mp.weixin.qq.com/s/_QTUGTAWLreUVcNJcVKBjA)
  > pallet写好后需要通过runtime加载到链上（就是runtime/src/lib.rs中的construct_runtime宏包含的部分）。
  > 那么对应到我们的测试，如果对pallet进行测试，我们也需要构建一个runtime测试环境，然后在这个环境中加载pallet，对pallet进行测试。
  > 所以，编写pallet的测试就分为以下几部分：
    1. 编写 mock runtime;
    2. 编写pallet的genesisconfig;
    3. 编写测试。

    - [文档资料](https://docs.substrate.io/v3/runtime/custom-rpcs/)

### 为某些trait提供默认实现。

- [为pallet编写tests](https://web.archive.org/web/20220627101811/https://mp.weixin.qq.com/s/ZU5SYYrL6OORWGEbRev7Zg)

## 智能合约

### 初探ink!

- [初探ink!智能合约开发](https://web.archive.org/web/20220628070102/https://mp.weixin.qq.com/s/tvva1vogIcyN60tWcBPqww)
    - [发展两年的波卡智能合约语言ink!将会带来什么影响？ - 知乎](https://web.archive.org/web/20220628095113/https://zhuanlan.zhihu.com/p/468304741)
      > ink!是由Parity在这里开发智能合约语言Rust编写智能合同并编译成Wasm代码。
      > 智能合同是在分散区块链网络上运行的计算机协议，可视为可自动执行的应用程序。
      > ink!它不同于其他更成熟的智能合约语言Parity的烙印。ink!最初是通过使用Rust宏系统生成自定义语法和风格，开发智能合约。
      > 但是这种方法偏离了Rust在不牺牲易用性或开发性的前提下，开发人员熟悉和喜欢的语言诞生了ink!2.0.为开发人员提供最大的灵活性。简单来说，整合后现在ink!所有的结构和语法都是纯的Rust
    - [参考](https://docs.substrate.io/tutorials/v3/ink-workshop/pt1/)

### 深入ink!

- [深入ink!智能合约](https://web.archive.org/web/20220628070203/https://mp.weixin.qq.com/s/lPkXiu4hyYryHCqx-peRmQ)
  > 学习智能合约的开发，主要包括：
    - ink!智能合约的结构；
    - 存储单个的值和hash map；
    - 安全的设置或获取这些值；
    - 编写public和private函数；
    - 配置Rust使用安全的数学库。
  > 总的来说，写ink！合约和直接用Rust编码没有太大的区别，只要能使用Rust都能很快的编写合约。

### ERC20

- [ERC20合约开发](https://web.archive.org/web/20220628065106/https://mp.weixin.qq.com/s/fLZc_lQxe1jpgfHmmZ65Lg)
    - [ERC20协议是什么意思?一文读懂ERC20协议_区块链技术_区块链_脚本之家](https://web.archive.org/web/20220628095517/https://www.jb51.net/blockchain/797814.html)
  > 学习写一个ERC20合约，主要包括：
    - 初始token设置；
    - 支持transfer；
    - 通过substrate触发事件。

## 连接其他链

### 中继链连接

- [Relay Chain](https://web.archive.org/web/20220628065121/https://mp.weixin.qq.com/s/QSJRjdvqKgbY9eMbdhpVOA)

> 学习启动一个relay chain， 通过cumus来创建自己的parachain，并且在在本地测试网络中将parachain连接到relaychain。

### 平行链连接

- [连接parachain](https://web.archive.org/web/20220628074818/https://mp.weixin.qq.com/s/hF3vevPi4ZLuXwR2Ehhp_A)
  > 上一节，我们启动了relaychain的节点，本节将连接parachain到relaychain节点。 主要包括以下内容：
    - 启动parachain；
    - parachain注册；
    - 和parachain交互；
    - 连接到添加的parachain节点。

## 测试

### 编写测试

- [learn-substrate-easy/9编写tests.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/9%E7%BC%96%E5%86%99tests.md)
- [为pallet编写benchmarking](https://web.archive.org/web/20220701080314/https://mp.weixin.qq.com/s/8DsbCwL8XkiIEbTlHx4oAg)
  > 为pallet编写benchmarking分两种情况，如下：
    1. 对函数进行性能测试时需要的构造条件不会涉及到本pallet以外的其它pallet；
    2. 在对函数进行性能测试时需要先使用其它的 pallet 构造测试的先决条件。
    ~~~admonish info title='大部分用第一种'
    - 第一种情况相对来说比较简单，这个也比较好找到例子。
    - 第二种情况则比较复杂，写起来也比较麻烦。
    - 不过在我们的开发中，大部分都是第一种情况。
    ~~~

### benchmarking

- [learn-substrate-easy/10编写benchmarking.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/10%E7%BC%96%E5%86%99benchmarking.md)
- [learn-substrate-easy/12编写复杂的benchmarking.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/12%E7%BC%96%E5%86%99%E5%A4%8D%E6%9D%82%E7%9A%84benchmarking.md)

## 升级

- [learn-substrate-easy/11升级runtime.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/11%E5%8D%87%E7%BA%A7runtime.md)
- [learn-substrate-easy/12升级substrate版本.md at main · KuanHsiaoKuo/learn-substrate-easy](https://github.com/KuanHsiaoKuo/learn-substrate-easy/blob/main/12%E5%8D%87%E7%BA%A7substrate%E7%89%88%E6%9C%AC.md)
- [无分叉runtime升级](https://web.archive.org/web/20220628065113/https://mp.weixin.qq.com/s/qvqsKqn_h79Uig30jgQk7A)
  > substrate框架的特性之一就是支持无分叉运行时升级。无分叉升级时以区块链自身能力支持和保护的方式增强区块链运行时的一种手段，区块链的运行时定义了区块链可以保持的状态，还定义了改变该装填的逻辑。
  > substrate可以在不分叉的情况下更新runtime，因为运行时的定义本身就是substrate链中的一个元素，网络参与者可以通过交易函数，特别是set_code函数来更新该值。
  > 由于运行时状态的更新受到区块链共识机制和加密安全的约束，网络参与者可以在不分叉的情况下使用不受信任分发的更新或者扩展的运行时逻辑，甚至不需要发布新的区块链客户端。
    - 本节内容：
        1. 使用sudo调用将schelduler
        2. pallet包含到runtime中；
        3. 调用runtime升级。