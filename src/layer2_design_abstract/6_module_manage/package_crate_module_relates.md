# 模块相关

<!--ts-->
* [模块相关](#模块相关)
   * [厘清Workspace、Package、crate和module的关系](#厘清workspacepackagecrate和module的关系)
      * [Package: 包含Cargo.toml](#package-包含cargotoml)
      * [workspace与package](#workspace与package)
      * [crate: 主要在Cargo.toml的[bin]/[lib]中指明](#crate-主要在cargotoml的binlib中指明)
      * [再来对比workspace、package和crate](#再来对比workspacepackage和crate)
      * [module](#module)
   * [模块呈现方式](#模块呈现方式)
      * [嵌套模块](#嵌套模块)
      * [文件模块](#文件模块)
      * [目录模块](#目录模块)
   * [隐私与导入导出](#隐私与导入导出)
      * [隐私管理](#隐私管理)
      * [嵌套导入](#嵌套导入)
      * [再次导出](#再次导出)
   * [参考资源](#参考资源)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: kuanhsiaokuo, at: Tue Jun 21 20:11:33 CST 2022 -->

<!--te-->

## 厘清Workspace、Package、crate和module的关系

### Package: 包含Cargo.toml

package就是cargo new的产物，里面包含一个cargo.toml，包名就写在里面的package里。比如substrate的一个包代码：

> [substrate/Cargo.toml at master · paritytech/substrate](https://github.com/paritytech/substrate/blob/master/client/allocator/Cargo.toml)

```toml
[package]
name = "sc-allocator"
version = "4.1.0-dev"
authors = ["Parity Technologies <admin@parity.io>"]
edition = "2021"
license = "Apache-2.0"
homepage = "https://substrate.io"
repository = "https://github.com/paritytech/substrate/"
description = "Collection of allocator implementations."
documentation = "https://docs.rs/sc-allocator"
readme = "README.md"

[package.metadata.docs.rs]
targets = ["x86_64-unknown-linux-gnu"]

[dependencies]
log = "0.4.17"
thiserror = "1.0.30"
sp-core = { version = "6.0.0", path = "../../primitives/core" }
sp-wasm-interface = { version = "6.0.0", path = "../../primitives/wasm-interface" }
```

1. package表明该package的基本信息
2. dependencies表示该package依赖的其他package

### workspace与package

> [Workspaces - The Cargo Book](https://doc.rust-lang.org/cargo/reference/workspaces.html)
默认情况下，一个Cargo.toml只能指明一个package，但是在workspace里面就可以指明多个，比如substrate的根cargo：
> [substrate/Cargo.toml at master · paritytech/substrate](https://github.com/paritytech/substrate/blob/master/Cargo.toml)

```toml
[workspace]
resolver = "2"

members = [
    "bin/node-template/node",
]
[profile.dev.package]
blake2 = { opt-level = 3 }
```

- workspace+members: 并发代表当前package包含的所有subpackage，只是指明一个工作区的所有package

> A Cargo.toml file can simultaneously define a package and a workspace to which it belongs, but that package is still a member of that workspace, not the other way around.

### crate: 主要在Cargo.toml的[bin]/[lib]中指明

[Cargo Targets - The Cargo Book](https://doc.rust-lang.org/cargo/reference/cargo-targets.html?highlight=bin#library)

```admonish tip title='crate'
A crate is the [lib] or [[bin]] tables in the Cargo.toml. 
At most one lib crate may be present, but an arbitrary number of bin crates may be present. 

You won't see these tables added explicitly too often, 
because they're implicitly present if you have src/lib.rs (lib crate) and/or src/main.rs (bin crate).
```

```toml
# Example of customizing the library in Cargo.toml.
[lib]
crate-type = ["cdylib"]
bench = false
```

```toml
# Example of customizing binaries in Cargo.toml.
[[bin]]
name = "cool-tool"
test = false
bench = false

[[bin]]
name = "frobnicator"
required-features = ["frobnicate"]

```

### 再来对比workspace、package和crate

```admonish info title='hierarchy'
Workspace → Package → Crate
```

Generally, a package exposes only one crate. Most library crates don't even have an associated binary crate(s) in their
package. It's due to this that package/crate terminology is often used interchangeably; for lib crates it is in the 90%
case. Package is also a generic term that people not familiar with Rust's ecosystem can understand, where crate is a
Rust-specific piece of jargon.

The two concepts are still meaningfully different -- while conventionally package and lib crate have the same name (
modulo hyphens vs underscores), this is not required in any way -- but for the most part there isn't an appreciable
difference unless you're paying attention to the weeds and edge cases.

### module

在rust中，module(模块)更多还是一种逻辑上的概念，主要使用mod关键字，下面会具体说说

## 模块呈现方式

### 嵌套模块

### 文件模块

### 目录模块

## 隐私与导入导出

### 隐私管理

### 嵌套导入

### 再次导出

```admonish info title="pub(crate) fn fn_name() {}"
Rust 中元素的隐私性是从模块层面开始的。作为程序库的作者,要从模块向用户公开一些内容可以使用关键字 pub。但是对于有一些元素,我们只想暴露给软件包中的其他模块,而不是用户。在这种情况下,我们可以对元素使用 pub(crate)修饰符,这允许元素仅在软件包内部暴露
```

## 参考资源

- <精通Rust(第二版)>-2.2模块
- <精通Rust(第二版)>-7.9 模块、路径和导入
- [Visibility and privacy - The Rust Reference](https://doc.rust-lang.org/stable/reference/visibility-and-privacy.html)
- [pub(in path), pub(crate), pub(super), and pub(self) - The Rust Reference](https://doc.rust-lang.org/stable/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself)
- [了解下Rust 模块使用方式](https://web.archive.org/web/20220620093333/https://mp.weixin.qq.com/s/mQ0zh_tcLEZZNpGIbz6BVA)
- [Managing Growing Projects with Packages, Crates, and Modules - The Rust Programming Language](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Modules - Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/mod.html)
- [Crates - Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/crates.html)
- [Confused about Package vs. Crate terminology. : rust](https://www.reddit.com/r/rust/comments/lvtzri/confused_about_package_vs_crate_terminology/)