# 模块相关

<!--ts-->
* [模块相关](#模块相关)
   * [模块方式](#模块方式)
      * [嵌套模块](#嵌套模块)
      * [文件模块](#文件模块)
      * [目录模块](#目录模块)
   * [嵌套导入](#嵌套导入)
   * [再次导出](#再次导出)
   * [隐私管理](#隐私管理)
   * [参考资源](#参考资源)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Jun 16 02:09:04 UTC 2022 -->

<!--te-->

## 模块方式

### 嵌套模块

### 文件模块

### 目录模块

## 嵌套导入

## 再次导出

## 隐私管理

```admonish info title="pub(crate) fn fn_name() {}"
Rust 中元素的隐私性是从模块层面开始的。作为程序库的作者,要从模块向用户公开一些内容可以使用关键字 pub。但是对于有一些元素,我们只想暴露给软件包中的其他模块,而不是用户。在这种情况下,我们可以对元素使用 pub(crate)修饰符,这允许元素仅在软件包内部暴露
```

## 参考资源

- <精通Rust(第二版)>-2.2模块
- <精通Rust(第二版)>-7.9 模块、路径和导入
- [Visibility and privacy - The Rust Reference](https://doc.rust-lang.org/stable/reference/visibility-and-privacy.html)
- [pub(in path), pub(crate), pub(super), and pub(self) - The Rust Reference](https://doc.rust-lang.org/stable/reference/visibility-and-privacy.html#pubin-path-pubcrate-pubsuper-and-pubself)