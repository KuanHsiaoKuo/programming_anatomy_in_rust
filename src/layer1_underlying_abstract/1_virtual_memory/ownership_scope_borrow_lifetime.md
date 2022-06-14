# 所有权四件套：所有权、作用域、借用与生命周期

![Ownership](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/Ownership.jpeg)

<!--ts-->
* [所有权四件套：所有权、作用域、借用与生命周期](#所有权四件套所有权作用域借用与生命周期)
   * [综述](#综述)
   * [所有权](#所有权)
   * [作用域](#作用域)
   * [借用](#借用)
   * [生命周期](#生命周期)
   * [参考资源](#参考资源)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Jun 14 05:37:06 UTC 2022 -->

<!--te-->

## 综述

```text
┌───────────────────────────────────┐
│                                   │
│                                   │
│         .───────.       ┌──────┐  │
│       ,'         `.     │Borrow│  │
│     ,'             `.   └──────┘  │
│    ;   ┌─────────┐   :            │
│    │   │OwnerShip│   │            │
│    │   └─────────┘   │            │
│    :     ┌─────┐     ; ┌────────┐ │
│     ╲    │Scope│    ╱  │Lifetime│ │
│      `.  └─────┘  ,'   └────────┘ │
│        `.       ,'                │
│          `─────'                  │
│                                   │
│                                   │
└───────────────────────────────────┘
```

## 所有权

## 作用域

## 借用

## 生命周期

## 参考资源

- [Object lifetime and ownership](https://www.ditsing.com/object-lifetime-and-ownership/)
- [What is Ownership? - The Rust Programming Language](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- <精通rust(第二版)>-5.7 内存安全三原则