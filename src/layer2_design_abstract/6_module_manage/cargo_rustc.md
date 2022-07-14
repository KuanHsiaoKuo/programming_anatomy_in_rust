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
   * [Cargo项目结构](#cargo项目结构)
      * [基础说明](#基础说明)
      * [cargo.toml和cargo.lock](#cargotoml和cargolock)
      * [构建、清理、更新以及安装](#构建清理更新以及安装)
      * [main.rs 和 lib.rs](#mainrs-和-librs)
      * [实际应用：Substrate的substrate-node-template](#实际应用substrate的substrate-node-template)
         * [区别cargo run与cargo build](#区别cargo-run与cargo-build)
         * [在node包里面存在main.rs](#在node包里面存在mainrs)
   * [参考资源](#参考资源)
      * [online-book](#online-book)
      * [fragment](#fragment)
      * [local](#local)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Jul 14 08:50:34 UTC 2022 -->

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

## Cargo项目结构

### 基础说明

![hello_world_project_sturcture](https://web.archive.org/web/20220621034241im_/https://pp88.org/rust.zui.jia.shi.jian.rust.zui.jia.shi.jian/cargo.xiang.mu.pei.zhi.ji.jie.gou/project-structure.png)

- cargo.toml和cargo.lock文件总是位于项目根目录下。
- 源代码位于src目录下。
- 默认的库入口文件是src/lib.rs。
- 默认的可执行程序入口文件是src/main.rs。
- 其他可选的可执行文件位于src/bin/*.rs(这里每一个rs文件均对应一个可执行文件)。
- 外部测试源代码文件位于tests目录下。
- 示例程序源代码文件位于examples。
- 基准测试源代码文件位于benches目录下。

### cargo.toml和cargo.lock

cargo.toml和cargo.lock是cargo项目代码管理的核心两个文件，cargo工具的所有活动均基于这两个文件。

cargo.toml是cargo特有的项目数据描述文件，对于猿们而言，cargo.toml文件存储了项目的所有信息，它直接面向rust猿，猿们如果想让自己的rust项目能够按照期望的方式进行构建、测试和运行，那么，必须按照合理的方式构建’cargo.toml’。

而cargo.lock文件则不直接面向猿，猿们也不需要直接去修改这个文件。lock文件是cargo工具根据同一项目的toml文件生成的项目依赖详细清单文件，所以我们一般不用不管他，只需要对着cargo.toml文件撸就行了。

### 构建、清理、更新以及安装

> 领会了toml描述文件的写法，是一个重要的方面。另一个重要的方面，就是cargo工具本身为我们程序猿提供的各种好用的工具。如果大家感兴趣，自己在终端中输入’cargo –help’查看即可。

1. 其中开发时最常用的命令就是'cargo build'，用于构建项目。
2. 此外，'cargo clean'命令可以清理target文件夹中的所有内容；
3. 'cargo update'根据toml描述文件重新检索并更新各种依赖项的信息，并写入lock文件，例如依赖项版本的更新变化等等；
4. 'cargo install'可用于实际的生产部署。这些命令在实际的开发部署中均是非常有用的。

### main.rs 和 lib.rs

~~~admonish info title='main.rs vs lib.rs'
```text
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓                                              
┃                                     _                                        ┃                                              
┃                          _ __  __ _(_)_ _    _ _ ___                         ┃                                              
┃                         | '  \/ _` | | ' \ _| '_(_-<                         ┃                                              
┃                         |_|_|_\__,_|_|_||_(_)_| /__/                         ┃                                              
┃                                                                              ┃                                              
┃ _                 _ _                             _             _   _        ┃                                              
┃| |_  __ _ _ _  __| | |___ ___  _ _ _  _ _ _  _ _ (_)_ _  __ _  | |_| |_  ___ ┃                                              
┃| ' \/ _` | ' \/ _` | / -_|_-< | '_| || | ' \| ' \| | ' \/ _` | |  _| ' \/ -_)┃                                              
┃|_||_\__,_|_||_\__,_|_\___/__/ |_|  \_,_|_||_|_||_|_|_||_\__, |  \__|_||_\___|┃                                              
┃                                                         |___/                ┃                                              
┃                                                                              ┃                                              
┃                       _ __ _ _ ___  __ _ _ _ __ _ _ __                       ┃                                              
┃                      | '_ \ '_/ _ \/ _` | '_/ _` | '  \                      ┃                                              
┃                      | .__/_| \___/\__, |_| \__,_|_|_|_|                     ┃                                              
┃                      |_|           |___/                                     ┃                                              
┃                                                                              ┃                                              
┃1. cargo new --bin                                                            ┃                                              
┃- Calling the command line parsing logic with the argument values             ┃                                              
┃- Setting up any other configuration                                          ┃                                              
┃- Calling a run function in lib.rs           ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┻━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃- Handling the error if run returns an error ┃All are rust basic entry:       ╳    _ _ _                                    ┃
┃                                             ┃1. main.rs is the running entry ╳   | (_) |__   _ _ ___                       ┃
┃                                             ┃2. lib.rs is the library entry  ╳   | | | '_ \_| '_(_-<                       ┃
┃                                             ┃╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳╳    |_|_|_.__(_)_| /__/                       ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫                                                                              ┃
                                              ┃   _                 _ _               _ _   _   _          _           _     ┃
                                              ┃  | |_  __ _ _ _  __| | |___ ___  __ _| | | | |_| |_  ___  | |___  __ _(_)__  ┃
                                              ┃  | ' \/ _` | ' \/ _` | / -_|_-< / _` | | | |  _| ' \/ -_) | / _ \/ _` | / _| ┃
                                              ┃  |_||_\__,_|_||_\__,_|_\___/__/ \__,_|_|_|  \__|_||_\___| |_\___/\__, |_\__| ┃
                                              ┃                                                                  |___/       ┃
                                              ┃        __   _   _          _           _          _     _                 _  ┃
                                              ┃   ___ / _| | |_| |_  ___  | |_ __ _ __| |__  __ _| |_  | |_  __ _ _ _  __| | ┃
                                              ┃  / _ \  _| |  _| ' \/ -_) |  _/ _` (_-< / / / _` |  _| | ' \/ _` | ' \/ _` | ┃
                                              ┃  \___/_|    \__|_||_\___|  \__\__,_/__/_\_\ \__,_|\__| |_||_\__,_|_||_\__,_| ┃
                                              ┃                                                                              ┃
                                              ┃                                                                              ┃
                                              ┃                                                                              ┃
                                              ┃                                                                              ┃
                                              ┃                                                                              ┃
                                              ┃                                                                              ┃
                                              ┃                                                                              ┃
                                              ┃                                                                              ┃
                                              ┃                                                                              ┃
                                              ┃                                                                              ┃
                                              ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```
~~~

- [Rust 模块系统 - 掘金](https://web.archive.org/web/20220703121446/https://juejin.cn/post/6919738138135003150)
  > 包（package）创建规则：

      - 一个包中至多只能包含一个库Crate。
      - 包中可以包含任意多个二进制Crate。
      - 包中至少包含一个 crate，无论是库的还是二进制的。
      - 包中应该包含一个 Cargo.toml 配置文件，用来说明如何去构建这些 crate。
  > 示例一：cargo new --bin 创建一个包含 二进制Crate 的包
  > 示例二：cargo new --lib 创建一个包含库(lib)crate的包
  > 示例三：--lib和--bin不可以同时使用，这种情况下可以示例一的基础上自己创建一个lib.rs文件
    - 一般情况下，我们将与程序运行相关的代码放在 main.rs 文件，其他真正的任务逻辑放在 lib.rs 文件中。
- [Rust modules confusion when there is main.rs and lib.rs - Stack Overflow](https://web.archive.org/web/20220703121604/https://stackoverflow.com/questions/57756927/rust-modules-confusion-when-there-is-main-rs-and-lib-rs)

> 这里有人详细介绍了只有lib.rs/main.rs的情况下，执行cargo run的情况。没有main.rs的时候，cargo run会告诉你：error: a bin target must be available for `cargo run`

- [main.rs vs lib.rs : rust](https://www.reddit.com/r/rust/comments/c41iph/mainrs_vs_librs/)

> A common pattern, even for binary-only crates, is to declare a "library" providing the majority of functionality, and then have main.rs just implement argument parsing and use that library.
> This allows for integration tests written against the library, which might otherwise be hard to do with only main.rs.

- [Main.rs and lib.rs at same level - help - The Rust Programming Language Forum](https://web.archive.org/web/20220703121647/https://users.rust-lang.org/t/main-rs-and-lib-rs-at-same-level/42499)
  > 这里有人跟着官方文档制作命令行工具时发现同样的疑惑点，底下有人引用了这个链接
    - [Separation of Conerns for Binary Projects - The Rust Programming Language](https://web.archive.org/web/20220424212816/https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#separation-of-concerns-for-binary-projects)

```admonish quote title='Allocating Responsibilty for Multiple Tasks'
The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. The process has the following steps:

1. Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
2. As long as your command line parsing logic is small, it can remain in main.rs.
3. When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

> The responsibilities that remain in the main function after this process should be limited to the following:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in lib.rs
- Handling the error if run returns an error

> This pattern is about separating concerns:
1. main.rs handles running the program
2. and lib.rs handles all the logic of the task at hand. 

> Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs. 
> The only code that remains in main.rs will be small enough to verify its correctness by reading it. 
> Let’s rework our program by following this process.
```

```admonish tip title='明确职责, 用MVC类比'
> 也就是说
1. main.rs只负责调用、配置和处理异常， 类似View
2. lib.rs主要负责具体的业务逻辑, 类似Control+Model
```

```admonish quote title='用crate来区分'
Note that the main.rs does not declare any modules with mod, it only imports them with use (the distinction between the two is something a lot of new Rust programmers struggle with!).

The reason for this is that, in projects with both a lib.rs and a main.rs:
> Cargo effectively treats lib.rs as the root of your crate, and main.rs as a seperate binary that depends on your crate.
```

```admonish quote title='用binary和library来区分'
**main.rs is always the root of your binary, and lib.rs is always the root of your library.**

> The important things to realize are:

- These two roots are compiled seperately, and have their own entirely seperate module structure.
- When you use the mod keyword, you are creating a module, not importing a module.
- You almost never want to have multiple mod statements for a single file, as you'll be duplicating the content.
- If you want to use something from an existing module or from a library, that's what the use keyword is for.

> Let's walk through your example:

1. In lib.rs, you use **mod** to declare that your library has two top level modules, cli and internal. There are corresponding files in the right place in your project, so Cargo knows how to link it all up. This part is fine, and what you intended to happen!
2. However, when you write mod lib in your main.rs, you're not importing your library! You're actually declaring that your binary has it's own top level module, which just happens to be called lib. Cargo then sees that there is a file called lib.rs in the folder, and links that in, even though that's not what you intended. It then sees that the lib.rs file has two mod statements and treats these as submodules of the binary's lib module. The files aren't in the right place for that to work, so compilation fails.
3. So the root cause of your issue is that you've effectively told the compiler to create your module structure twice, **once in your library and once in your binary**. 
4. You really only meant to define these modules in your library, and then use them into your binary.

```

### 实际应用：Substrate的substrate-node-template

> 根据官方文档知道，substrate是各种lib包，那么开发时如何运行呢？

#### 区别cargo run与cargo build

- [构建并运行Cargo - Rust 程序设计语言 简体中文版](https://kaisery.github.io/trpl-zh-cn/ch01-03-hello-cargo.html#%E6%9E%84%E5%BB%BA%E5%B9%B6%E8%BF%90%E8%A1%8C-cargo-%E9%A1%B9%E7%9B%AE)

#### 在node包里面存在main.rs

1. 根Cargo.toml里面告诉cargo，这个项目总共有三个包

~~~admonish info title='根Cargo.toml'
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

2. 然后cargo找到包含main.rs的node包作为入口

~~~admonish tip title='node tree'
```shell
 tree -L 2 node                                                                                                                                                                                                                         ─╯
node
├── Cargo.toml
├── build.rs
└── src
    ├── chain_spec.rs
    ├── cli.rs
    ├── command.rs
    ├── command_helper.rs
    ├── lib.rs
    ├── main.rs
    ├── rpc.rs
    └── service.rs

1 directory, 10 files
```
~~~

3. node/main.rs里面使用了文件名crate

```rust
//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
// 对应service.rs
mod service;
// 对应cli.rs
mod cli;
// 对应command.rs
mod command;
// 对应command_helper.rs
mod command_helper;
// 对应rpc.rs
mod rpc;

fn main() -> sc_cli::Result<()> {
    command::run()

```

## 参考资源

### online-book

- [What is rustc? - The rustc book](https://doc.rust-lang.org/rustc/what-is-rustc.html)

### fragment

- [Rustc | Rust学习笔记](https://skyao.io/learning-rust/docs/build/rustc.html)
- [Cargo项目配置及结构 | 皮皮爸(pp8)](https://web.archive.org/web/20220621034241/https://pp88.org/rust.zui.jia.shi.jian.rust.zui.jia.shi.jian/cargo.xiang.mu.pei.zhi.ji.jie.gou/)

### local
