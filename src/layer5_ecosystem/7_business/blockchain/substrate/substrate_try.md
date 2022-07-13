# 尝试Substrate

<!--ts-->
* [尝试Substrate](#尝试substrate)
   * [初体验Substrate链](#初体验substrate链)
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
   * [参考资源](#参考资源)
      * [substrate文档练习](#substrate文档练习)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Jul 13 06:58:48 UTC 2022 -->

<!--te-->

## 初体验Substrate链

这里主要使用官方提供的默认模版启动节点。

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

> Output JSON to stdout containing information about the workspace members and resolved dependencies of the current package.

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

### 使用前端模版

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

## Substrate使用方式

> 使用substrate的方式主要有以下几种：

### 使用subtrate node

开发者可以运行已经设计好的substrate节点，并配置genesis区块，在此方式下只需要提供一个json文件就可以启动自己的区块链。其实我们上一节的substrate初体验，也可以看成是使用此种方式的一个例子。

### 使用substrate frame

frame其实是一组模块（pallet）和支持库。使用substrate frame可以轻松的创建自己的自定义运行时，因为frame是用来构建底层节点的。使用frame还可以配置数据类型，也可以从模块库中选择甚至是添加自己定义的模块。

### 使用substrate core

使用substrate code运行开发者完全从头开始设计运行时（runtime，问题：什么是runtime？），当然此种方式也是使用substrate自由度最大的方式。

```admonish tip title='几种方式的关系可以用图描述如下：技术自由 vs 开发便利'
![Technical freedom vs development ease](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/technical-freedom.png)
```

## 参考资源

### substrate文档练习

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
           > frame其实是一组模块（pallet）和支持库。使用substrate frame可以轻松的创建自己的自定义运行时，因为frame是用来构建底层节点的。使用frame还可以配置数据类型，也可以从模块库中选择甚至是添加自己定义的模块。
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
      > node template的运行时是基于FRAME来实现的。FRAME是一个代码库，允许我们使用一系列pallet来构建底层的运行时。pallet可以看出是定义区块链功能的单个逻辑模块。subtrate为我们提供了一些已经构建好的pallet，我们在定义运行时时可以直接使用。
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
- [初探ink!智能合约开发](https://web.archive.org/web/20220628070102/https://mp.weixin.qq.com/s/tvva1vogIcyN60tWcBPqww)
    - [发展两年的波卡智能合约语言ink!将会带来什么影响？ - 知乎](https://web.archive.org/web/20220628095113/https://zhuanlan.zhihu.com/p/468304741)
      > ink!是由Parity在这里开发智能合约语言Rust编写智能合同并编译成Wasm代码。
      > 智能合同是在分散区块链网络上运行的计算机协议，可视为可自动执行的应用程序。
      > ink!它不同于其他更成熟的智能合约语言Parity的烙印。ink!最初是通过使用Rust宏系统生成自定义语法和风格，开发智能合约。
      > 但是这种方法偏离了Rust在不牺牲易用性或开发性的前提下，开发人员熟悉和喜欢的语言诞生了ink!2.0.为开发人员提供最大的灵活性。简单来说，整合后现在ink!所有的结构和语法都是纯的Rust
    - [参考](https://docs.substrate.io/tutorials/v3/ink-workshop/pt1/)
- [深入ink!智能合约](https://web.archive.org/web/20220628070203/https://mp.weixin.qq.com/s/lPkXiu4hyYryHCqx-peRmQ)
  > 学习智能合约的开发，主要包括：
    - ink!智能合约的结构；
    - 存储单个的值和hash map；
    - 安全的设置或获取这些值；
    - 编写public和private函数；
    - 配置Rust使用安全的数学库。
  > 总的来说，写ink！合约和直接用Rust编码没有太大的区别，只要能使用Rust都能很快的编写合约。
- [ERC20合约开发](https://web.archive.org/web/20220628065106/https://mp.weixin.qq.com/s/fLZc_lQxe1jpgfHmmZ65Lg)
    - [ERC20协议是什么意思?一文读懂ERC20协议_区块链技术_区块链_脚本之家](https://web.archive.org/web/20220628095517/https://www.jb51.net/blockchain/797814.html)
  > 学习写一个ERC20合约，主要包括：
    - 初始token设置；
    - 支持transfer；
    - 通过substrate触发事件。
- [无分叉runtime升级](https://web.archive.org/web/20220628065113/https://mp.weixin.qq.com/s/qvqsKqn_h79Uig30jgQk7A)
  > substrate框架的特性之一就是支持无分叉运行时升级。无分叉升级时以区块链自身能力支持和保护的方式增强区块链运行时的一种手段，区块链的运行时定义了区块链可以保持的状态，还定义了改变该装填的逻辑。
  > substrate可以在不分叉的情况下更新runtime，因为运行时的定义本身就是substrate链中的一个元素，网络参与者可以通过交易函数，特别是set_code函数来更新该值。
  > 由于运行时状态的更新受到区块链共识机制和加密安全的约束，网络参与者可以在不分叉的情况下使用不受信任分发的更新或者扩展的运行时逻辑，甚至不需要发布新的区块链客户端。
    - 本节内容：
        1. 使用sudo调用将schelduler
        2. pallet包含到runtime中；
        3. 调用runtime升级。
- [Relay Chain](https://web.archive.org/web/20220628065121/https://mp.weixin.qq.com/s/QSJRjdvqKgbY9eMbdhpVOA)
  > 学习启动一个relay chain， 通过cumus来创建自己的parachain，并且在在本地测试网络中将parachain连接到relaychain。
- [连接parachain](https://web.archive.org/web/20220628074818/https://mp.weixin.qq.com/s/hF3vevPi4ZLuXwR2Ehhp_A)
  > 上一节，我们启动了relaychain的节点，本节将连接parachain到relaychain节点。 主要包括以下内容：
    - 启动parachain；
    - parachain注册；
    - 和parachain交互；
    - 连接到添加的parachain节点。 