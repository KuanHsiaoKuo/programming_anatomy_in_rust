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
            * [SS58: 对应公钥/地址格式](#ss58-对应公钥地址格式)
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
    * [Add a pallet to the runtime](#add-a-pallet-to-the-runtime)
        * [runtime结构分析](#runtime结构分析)
        * [时序图](#时序图-1)
    * [Specify the origin for a call](#specify-the-origin-for-a-call)
      * [为帐户设置昵称](#为帐户设置昵称)
      * [使用Nicks pallet查询账户信息](#使用nicks-pallet查询账户信息)
        * [可能出现的问题](#可能出现的问题)
        * [signed与sudo有不同权限。](#signed与sudo有不同权限)
    * [Configure the contracts pallet](#configure-the-contracts-pallet)
    * [Use macros in a custom pallet](#use-macros-in-a-custom-pallet)
        * [时序图](#时序图-2)
    * [Publish Custom pallets](#publish-custom-pallets)
        * [内置pallets](#内置pallets)
        * [发布pallets](#发布pallets)
* [Develop smart contracts](#develop-smart-contracts)
    * [Prepare your first contract](#prepare-your-first-contract)
    * [Develop a smart contract](#develop-a-smart-contract)
    * [Use maps for storing values](#use-maps-for-storing-values)
    * [Buid a token contract](#buid-a-token-contract)
    * [Troubleshoot smart contracts](#troubleshoot-smart-contracts)
* [Connect with other chains](#connect-with-other-chains)
    * [Start a local relay chain](#start-a-local-relay-chain)
        * [波卡架构](#波卡架构)
        * [parachains](#parachains)
        * [注意版本匹配](#注意版本匹配)
        * [时序图](#时序图-3)
    * [Connect a local parachain](#connect-a-local-parachain)
        * [Common Good Parachains](#common-good-parachains)
        * [Conver a solo chain](#conver-a-solo-chain)
        * [Parachain Slots Autcion](#parachain-slots-autcion)
        * [跨链用到的信息格式](#跨链用到的信息格式)
        * [时序图](#时序图-4)
    * [Connect to Rococo testnet](#connect-to-rococo-testnet)
        * [substrate预置账户和密钥](#substrate预置账户和密钥)
        * [wallets](#wallets)
        * [polkadot-js/extension](#polkadot-jsextension)
        * [SS58地址格式](#ss58地址格式)
        * [Rococo faucet martic channel](#rococo-faucet-martic-channel)
        * [时序图](#时序图-5)
    * [Access EVM accounts](#access-evm-accounts)
        * [Ethereum core concepts and terminology](#ethereum-core-concepts-and-terminology)
        * [Ethereum Virtual Machine (EVM) basics](#ethereum-virtual-machine-evm-basics)
        * [Decentralized applications and smart contracts](#decentralized-applications-and-smart-contracts)
        * [Pallet design principles](#pallet-design-principles)
        * [Truffle](#truffle)
        * [Remix IDE](#remix-ide)
        * [时序图](#时序图-6)
* [参考资源](#参考资源)
    * [substrate文档练习](#substrate文档练习)
    * [pallet基础](#pallet基础)
        * [尝试添加pallet到runtime](#尝试添加pallet到runtime)
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
<!-- Added by: runner, at: Thu Aug 25 14:30:43 UTC 2022 -->

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
{{#include ../../../../../materials/node-template-structure.txt:1:}}
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

#### SS58: 对应公钥/地址格式

- [pub use ss58_registry-substrate/crypto.rs at 0ba251c9388452c879bfcca425ada66f1f9bc802 · paritytech/substrate](https://github.com/paritytech/substrate/blob/0ba251c9388452c879bfcca425ada66f1f9bc802/primitives/core/src/crypto.rs#L46)
- [Glossary | Substrate_ Docs](https://docs.substrate.io/reference/glossary/#ss58-address-format)

### 步骤：

1. 使用Sr25519 -> 一个助记词和对应SS58公钥 -> aura
2. 使用Ed25519+前面的助记词 -> Ed25519 公钥 -> grandpa

### actdiag

![add trusted noted seq](kroki-actdiag:../../../../../materials/plantumls/substrate_tutorials/get-started/add-trusted-nodes-seq.actdiag)

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

[//]: # (![authorize specific nodes seq]&#40;kroki-mermaid:../../../../../materials/plantumls/substrate_tutorials/get-started/authorize-specific-nodes-seq.mermaid&#41;)

```mermaid
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
      - targets: [ "localhost:9615" ]
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

## Pallet前置Rust知识

{{#check Pallet-Preset| pallet 前置Rust知识}}

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

### 时序图

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/work-with-pallets/add-a-pallet-to-the-runtime.puml:1:}}
```

## Specify the origin for a call

> 此小节接着上节内容进行修改，主要是强化权限

- [Specify the origin for a call | Substrate_ Docs](https://docs.substrate.io/tutorials/work-with-pallets/specify-the-origin-for-a-call/)

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

## Configure the contracts pallet

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/work-with-pallets/configure-the-contracts-pallet.puml:1:}}
```

## Use macros in a custom pallet

> [返回顶部](#substrate官方教程梳理与练习)

```admonish info title='这一节干货较多'
1. 了解Substrate Runtime development： [Runtime development | Substrate_ Docs](https://docs.substrate.io/main-docs/fundamentals/runtime-intro/)
2. 尤其要理解FRAME和pallets的关系。
3. 掌握自定义pallet的步骤，其实已经准备好模版：substrate-node-template/pallets/template
4. 更多详细内容：[how-to-guides: pallet-design](https://docs.substrate.io/reference/how-to-guides/#pallet-design)
```

### 时序图

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/work-with-pallets/use-macros-in-a-custom-pallet.puml:1:}}
```

## Publish Custom pallets

- [Publish custom pallets | Substrate_ Docs](https://docs.substrate.io/tutorials/work-with-pallets/publish-custom-pallets/)

### 内置pallets

- [substrate/frame at master · paritytech/substrate](https://github.com/paritytech/substrate/tree/master/frame)

### 发布pallets

```admonish tip title='两种方法'
1. github发布
2. crates.io发布

这两个方法其实就是rust的crate常见发布方式。
- [模块系统 - Programming Anatomy In Rust 🦀](https://kuanhsiaokuo.github.io/programming_anatomy_in_rust/layer2_design_abstract/6_module_manage/module_system.html#%E6%95%B4%E7%90%86%E8%AF%B4%E4%B8%80%E4%B8%8Brust%E7%9A%84%E6%A8%A1%E5%9D%97%E7%B3%BB%E7%BB%9F)
```

# Develop smart contracts

## Prepare your first contract

> [返回顶部](#substrate官方教程梳理与练习)

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/develop-smart-contracts/prepare-your-first-contract.puml:1:}}
```

## Develop a smart contract

> [返回顶部](#substrate官方教程梳理与练习)

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/develop-smart-contracts/develop-a-smart-contract.puml:1:}}
```

## Use maps for storing values

> [返回顶部](#substrate官方教程梳理与练习)

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/develop-smart-contracts/use-maps-for-storing-values.puml:1:}}
```

## Buid a token contract

> [返回顶部](#substrate官方教程梳理与练习)

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/develop-smart-contracts/build-a-token-contract.puml:1:}}
```

## Troubleshoot smart contracts

> [返回顶部](#substrate官方教程梳理与练习)

# Connect with other chains

## Start a local relay chain

> [返回顶部](#substrate官方教程梳理与练习)

### 波卡架构

- [Architecture · Polkadot Wiki](https://wiki.polkadot.network/docs/learn-architecture)

### parachains

- [Parachains · Polkadot Wiki](https://wiki.polkadot.network/docs/learn-parachains)

### 注意版本匹配

> 您必须使用本教程中规定的确切版本。

平行链与它们连接的中继链代码库紧密耦合，因为它们共享许多共同的依赖关系。
在处理整个 Substrate 文档中的任何示例时，请务必将相应版本的 Polkadot
与任何其他软件一起使用。您必须与中继链升级保持同步，您的平行链才能继续成
功运行。如果你不跟上中继链升级，你的网络很可能会停止生产区块。

### 时序图

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/connect-with-other-chains/start-a-local-relay-chain.puml:1:k}}
```

## Connect a local parachain

> [返回顶部](#substrate官方教程梳理与练习)

### Common Good Parachains

- [Common Good Parachains · Polkadot Wiki](https://wiki.polkadot.network/docs/learn-common-goods)

### Conver a solo chain

- [Convert a solo chain | Substrate_ Docs](https://docs.substrate.io/reference/how-to-guides/parachains/convert-a-solo-chain/)

### Parachain Slots Autcion

- [Parachain Slots Auction · Polkadot Wiki](https://wiki.polkadot.network/docs/learn-auction)

### 跨链用到的信息格式

- [Cross-Consensus Message Format (XCM) · Polkadot Wiki](https://wiki.polkadot.network/docs/learn-crosschain)

### 时序图

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/connect-with-other-chains/connect-a-local-parachain.puml:1:k}}
```

## Connect to Rococo testnet

> [返回顶部](#substrate官方教程梳理与练习)

### substrate预置账户和密钥

- [subkey | Substrate_ Docs](https://docs.substrate.io/reference/command-line-tools/subkey/#predefined-accounts-and-keys)

### wallets

- [Wallets · Polkadot Wiki](https://wiki.polkadot.network/docs/build-wallets)

### polkadot-js/extension

- [polkadot-js/extension: Simple browser extension for managing Polkadot and Substrate network accounts in a browser. Allows the signing of extrinsics using these accounts. Also provides a simple interface for compliant extensions for dapps.](https://github.com/polkadot-js/extension)

### SS58地址格式

- [Glossary | Substrate_ Docs](https://docs.substrate.io/reference/glossary/#ss58-address-format)

### Rococo faucet martic channel

- [You're invited to talk on Matrix](https://matrix.to/#/#rococo-faucet:matrix.org)

### 时序图

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/connect-with-other-chains/connect-to-rococo-testnet.puml:1:}}
```

## Access EVM accounts

> [返回顶部](#substrate官方教程梳理与练习)

### Ethereum core concepts and terminology

### Ethereum Virtual Machine (EVM) basics

### Decentralized applications and smart contracts

### Pallet design principles

### Truffle

[Truffle - Truffle Suite](https://trufflesuite.com/truffle/)

### Remix IDE

[Remix - Ethereum IDE](http://remix.ethereum.org/#optimize=false&runs=200&evmVersion=null&version=soljson-v0.8.7+commit.e28d00a7.js)

### 时序图

```plantuml
{{#include ../../../../../materials/plantumls/substrate_tutorials/connect-with-other-chains/access-evm-accounts.puml:1:}}
```

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
