# Cargo与crate生命周期

![cargo_ship](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/cargo_ship.jpeg)

<!--ts-->


<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: kuanhsiaokuo, at: Sun Jul 10 18:30:12 CST 2022 -->

<!--te-->

## cargo <cmd>: 用生命周期理解Cargo指令系列

### 新建

#### cargo new

#### cargo init

### 开发

#### cargo clean

#### cargo doc

### 依赖管理

#### cargo check

该命令用来快速检查当前代码是否可以通过编译，但是不去生成真正可执行的程序。这样可以加快我们的检查速度。

#### cargo fix

#### cargo fetch

#### cargo search/install/uninstall

#### cargo report

#### cargo generate-lockfile

#### cargo locate-project

#### cargo metadata

#### cargo pkgid

#### cargo tree

#### cargo update

#### cargo vendor

#### cargo verify-project

### 测试

#### cargo test

#### cargo bench

### 运行

#### cargo run

### 构建

#### cargo rustc

#### cargo build

```shell
cargo build --release
```

> 该命令将会在 target/release/目录下生成优化过的可执行程序。这样生成的可执行程序拥有更好的性能。

#### cargo rustdoc

### 发布

#### cargo login

#### cargo owner

#### cargo package

#### cargo publish

#### cargo yank

### 维护

### 分区

## Cargo.toml细说

## Rust程序运行方式总结

### 脚本

### 项目

## 参考资源

### online-book

- [Cargo Commands - The Cargo Book](https://doc.rust-lang.org/cargo/commands/index.html)
- [Cargo - Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/cargo.html)

### fragment

### local

- [<精通Rust(第二版)>-2.3 Cargo和程序库](marginnote3app://note/607C0511-4592-4F59-A8C0-AD8017A503FE)
