<!--ts-->
   * [介绍](#介绍)
   * [Rust语言架构](#rust语言架构)
   * [Rust语言版本说明](#rust语言版本说明)
   * [Rust编译步骤](#rust编译步骤)
   * [Rust编译过程与宏展开](#rust编译过程与宏展开)
   * [参考资源](#参考资源)
      * [online-book](#online-book)
      * [fragment](#fragment)
      * [local](#local)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Jul 29 08:58:48 UTC 2022 -->

<!--te-->

## 介绍

从计算机组成原理了解到操作系统，当一个程序开始运行的时候，不论是可执行程序还是命令行，都会从创建进程，申请进程资源开始，再到堆栈(stack/heap)
的使用，申请与释放资源。这一系列操作对于编程来说重要性不言而喻，只不过根据编程语言的高级程度不同，开发者需要掌握的知识也有不同。

而在rust语言编程中，内存的管理方式及其重要。所以这一层主要先介绍虚拟内存管理以及相关出现的内容安全问题，接着介绍rust是如何通过所有权、作用域和生命周期，引申出借用、移动语义、复制语义等一系列内容来解决内存安全问题。

## Rust语言架构

![rust language architecture](kroki-excalidraw:../../../materials/rust_basics/rust-language-architecture.excalidraw)

## Rust语言版本说明

![image-20220728101929348](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220728101929348.png)

## Rust编译步骤

![rust compiling](kroki-excalidraw:../../../materials/rust_basics/rust-compiling.excalidraw)

## Rust编译过程与宏展开

> 这里主要参考下列内容

- [RustcRustc/rustc-dev-guide-zh: Rustc Dev Guide 中文翻译](https://github.com/RustcRustc/rustc-dev-guide-zh)
- [About this guide - Guide to Rustc Development](https://rustc-dev-guide.rust-lang.org/about-this-guide.html)
- [Rust编译器专题 | 图解 Rust 编译器与语言设计 Part 1 - Rust精选](https://rustmagazine.github.io/rust_magazine_2021/chapter_1/rustc_part1.html)

![rust compile process macro expand](kroki-excalidraw:../../../materials/rust_basics/rust-compile-process-macro-expand.excalidraw)

## 参考资源

### online-book

### fragment

### local