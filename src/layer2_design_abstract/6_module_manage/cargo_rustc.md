# cargo与rustc

<!--ts-->
* [cargo与rustc](#cargo与rustc)
   * [rustc](#rustc)
      * [rustc是什么](#rustc是什么)
      * [基础使用](#基础使用)
      * [rustc与cargo的关系](#rustc与cargo的关系)
   * [cargo style](#cargo-style)
   * [cargo essential structure](#cargo-essential-structure)
   * [.cargo 扩展](#cargo-扩展)
      * [tree overview](#tree-overview)
      * [bin](#bin)
      * [env](#env)
      * [git](#git)
      * [registry](#registry)
   * [Cargo 与 git 的关联！](#cargo-与-git-的关联)
      * [cargo tree](#cargo-tree)
      * [git](#git-1)
      * [关于依赖冲突问题](#关于依赖冲突问题)
   * [Cargo相关问题解决](#cargo相关问题解决)
      * [版本冲突：failed to select a version for the requirement](#版本冲突failed-to-select-a-version-for-the-requirement)
   * [参考资源](#参考资源)
      * [online-book](#online-book)
      * [fragment](#fragment)
      * [local](#local)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: kuanhsiaokuo, at: Sun Jul  3 19:16:06 CST 2022 -->

<!--te-->

## rustc

### rustc是什么

全称应该是"rust complier", 是 Rust 编程语言的编译器，由项目本身提供。编译器获取源代码并生成二进制代码，作为库或可执行文件。

### 基础使用

```shell
rustc target_file.rs
```

### rustc与cargo的关系

这其实就类似前端webpack.config和package.json的关系, cargo在运行时会默认调用rustc指令。

~~~admonish tip title='查看cargo如何调用rustc'
```shell
$ cargo build --verbose
```
> verbose：就是详细信息的意思，运行后将会打印出每次调用rustc的过程。
~~~

## cargo style

cargo可以看作是基于rustc的"管理框架"，主要还是对于项目用到的包的管理。

~~~admonish tip title='查看cargo'
[这里](/layer2_design_abstract/6_module_manage/cargo_crate_lifetime)可以看到具体有哪些功能
~~~

## cargo essential structure

但是对于大多数场景下，一般采用如下的项目结构

~~~admonish tip title='rust package structure'
```shell
$ tree use-benchmarking -L 2                                                                                                                                                                                                             ─╯
use-benchmarking
├── Cargo.toml
├── README.md
└── src
    ├── benchmarking.rs
    ├── lib.rs
    ├── mock.rs
    ├── tests.rs
    └── weights.rs

1 directory, 7 files
```
~~~

## .cargo 扩展

除了cargo, 它的全局相关配置文件都会放在$HOME/.cargo目录下，这里值得看看

### tree overview

```shell
tree -L 1 $HOME/.cargo                                                                                                                                                                                                                 ─╯
├── bin # 安装的二进制相关可执行文件
├── env # rustup的环境变量设置脚本
├── git
└── registry
```

### bin

```shell
tree -L 1 $HOME/.cargo/bin                                                                                                                                                                                                             ─╯
├── cargo -> /usr/local/bin/rustup-init
├── cargo-clippy -> /usr/local/bin/rustup-init
├── cargo-fmt -> /usr/local/bin/rustup-init
├── cargo-miri -> /usr/local/bin/rustup-init
├── clippy-driver -> /usr/local/bin/rustup-init
├── mdbook
├── mdbook-admonish
├── mdbook-checklist
├── mdbook-mermaid
├── mdbook-pdf
├── mdbook-rss
├── mdbook-template
├── rls -> /usr/local/bin/rustup-init
├── rust-gdb -> /usr/local/bin/rustup-init
├── rust-lldb -> /usr/local/bin/rustup-init
├── rustc -> /usr/local/bin/rustup-init
├── rustdoc -> /usr/local/bin/rustup-init
├── rustfmt -> /usr/local/bin/rustup-init
└── rustup -> /usr/local/bin/rustup-init

0 directories, 19 files
```

### env

```shell
#!/bin/sh
# rustup shell setup
# affix colons on either side of $PATH to simplify matching
case ":${PATH}:" in
    *:"$HOME/.cargo/bin":*)
        ;;
    *)
        # Prepending path in case a system-installed rustc needs to be overridden
        export PATH="$HOME/.cargo/bin:$PATH"
        ;;
esac
```

### git

```shell
tree -L 3 $HOME/.cargo/git                                                                                                                                                                                                             ─╯
/Users/kuanhsiaokuo/.cargo/git
├── checkouts
│   ├── substrate-7e08433d4c370a21
│   │   ├── 174735e
│   │   ├── 257cdb5
│   │   ├── 279593d
│   │   ├── 3348e14
│   │   ├── 346471d
│   │   ├── 4d28ebe
│   │   ├── 7ba4e4c
│   │   ├── 7eb671f
│   │   ├── 814752f
│   │   ├── 852bab0
│   │   ├── b6c1c1b
│   │   └── bf9683e
│   └── unveil-rs-403565214a7cc66c
│       └── 3d8e9ad
└── db
    ├── substrate-7e08433d4c370a21
    │   ├── FETCH_HEAD
    │   ├── HEAD
    │   ├── config
    │   ├── description
    │   ├── hooks
    │   ├── info
    │   ├── objects
    │   └── refs
    └── unveil-rs-403565214a7cc66c
        ├── FETCH_HEAD
        ├── HEAD
        ├── config
        ├── description
        ├── hooks
        ├── info
        ├── objects
        └── refs

27 directories, 8 files
```

### registry

```shell
tree -L 2 $HOME/.cargo/registry                                                                                                                                                                                                        ─╯
/Users/kuanhsiaokuo/.cargo/registry
├── cache
│   └── github.com-1ecc6299db9ec823
├── index
│   └── github.com-1ecc6299db9ec823
└── src
    └── github.com-1ecc6299db9ec823

6 directories, 0 files
```

## Cargo 与 git 的关联！
### cargo tree
1. Cargo.toml是分等级的，最外层的Cargo.toml里面可以只用一个members，然后里面列出内部包含的其他packages
~~~admonish info title='substrate-node-template为例'
> 最外层的Cargo.toml
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
~~~

2. 然后内部的其他packages就需要列出用到的[dependencies]
3. dependencies的完整使用参数如下：
~~~admonish info title='dependencies写法一'
```toml
[dependencies]
pallet-aura = { version = "4.0.0-dev", default-features = false, git = "https://github.com/paritytech/substrate.git", branch = "polkadot-v0.9.24" }
```
~~~
~~~admonish info title='dependencies写法二'
```toml
[dependencies.pallet-aura]
version = "4.0.0-dev"
default-features = false
git = "https://github.com/paritytech/substrate.git"
branch = "polkadot-v0.9.24"
```

> 这里的branch也可以用tag(git tag)
~~~
### git

> 上面dependencie里面的git+branch/tag唯一确定了一份代码，cargo下载对应的git代码之后，会从最外层Cargo.toml顺着members一层层找到[package]如下所示的Cargo.toml来确定对应的依赖包

~~~admonish tip title='substrate/frame/aura/Cargo.toml'
```toml
[package]
name = "pallet-aura"
version = "4.0.0-dev"
authors = ["Parity Technologies <admin@parity.io>"]
edition = "2021"
license = "Apache-2.0"
homepage = "https://substrate.io"
repository = "https://github.com/paritytech/substrate/"
description = "FRAME AURA consensus pallet"
readme = "README.md"
```
~~~

### 关于依赖冲突问题

目前Cargo无法解决依赖冲突问题，一般都会是因为dependencies里面的git+branch/tag对应的依赖更新导致。
```admonish tip title='顺藤摸瓜，一一排查'
这时需要根据冲突的包，切换分支/tag, 找到对应的branch/tag来更新
```

> 应用场景: [substrate添加pallet](/layer5_ecosystem/7_business/blockchain/substrate/substrate_deep_try.html#添加依赖-cargotomldependincies)

## Cargo相关问题解决

### 版本冲突：failed to select a version for the requirement

- 删除registry
  [How to fix "failed to select a version for the requirement" in Rust's Cargo - Manual](https://web.archive.org/web/20220702104818/https://blog.illixion.com/2021/10/fix-failed-to-select-a-version-cargo/)

```admonist info title='解决方案'
Apparently, Cargo can sometimes get into a state where its local registry cache will corrupt itself, and Cargo won’t be able to discover new versions of the dependencies. To resolve this, delete the ~/.cargo/registry folder and run the build command again.
```

- 修改dependencies
  [cargo install 出现需求版本选择失败怎么办--Qiita](https://web.archive.org/web/20220702110637/https://qiita.com/bc_yuuuuuki/items/6f566ddef60a201af1bc)

## 参考资源

### online-book

- [What is rustc? - The rustc book](https://doc.rust-lang.org/rustc/what-is-rustc.html)

### fragment

- [Rustc | Rust学习笔记](https://skyao.io/learning-rust/docs/build/rustc.html)

### local
