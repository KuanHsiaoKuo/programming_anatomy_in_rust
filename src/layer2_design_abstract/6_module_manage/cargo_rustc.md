# cargo与rustc

<!--ts-->
* [cargo与rustc](#cargo与rustc)
   * [rustc](#rustc)
      * [rustc是什么](#rustc是什么)
      * [基础使用](#基础使用)
      * [rustc与cargo的关系](#rustc与cargo的关系)
   * [cargo style](#cargo-style)
   * [cargo essential structure](#cargo-essential-structure)
   * [参考资源](#参考资源)
      * [online-book](#online-book)
      * [fragment](#fragment)
      * [local](#local)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: kuanhsiaokuo, at: Fri Jul  1 21:11:16 CST 2022 -->

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

## 参考资源

### online-book

- [What is rustc? - The rustc book](https://doc.rust-lang.org/rustc/what-is-rustc.html)

### fragment

- [Rustc | Rust学习笔记](https://skyao.io/learning-rust/docs/build/rustc.html)

### local
