# 代码质量：异常、测试与日志

<!--ts-->
* [代码质量：异常、测试与日志](#代码质量异常测试与日志)
   * [错误处理](#错误处理)
      * [RUST_BACKTRACE的用法](#rust_backtrace的用法)
         * [设置环境变量](#设置环境变量)
         * [临时设置使用](#临时设置使用)
   * [参考资源](#参考资源)
      * [online-book](#online-book)
      * [fragment](#fragment)
      * [local](#local)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Jul 20 07:33:13 UTC 2022 -->

<!--te-->

## 错误处理

### RUST_BACKTRACE的用法

- 1 : 打印简略错误信息
- full: 打印详细错误信息

#### 设置环境变量

```shell
export RUST_BACKTRACE=1
echo $RUST_BACKTRACE
```

#### 临时设置使用

```shell
RUST_BACKTRACE=1 cargo run
RUST_BACKTRACE=full mdbook serve
```

## 参考资源

### online-book

- [Writing Automated Tests - The Rust Programming Language](https://doc.rust-lang.org/book/ch11-00-testing.html)

### fragment

- [Rust 错误处理 | 菜鸟教程](https://www.runoob.com/rust/rust-error-handle.html)

### local
