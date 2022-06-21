# 编程解剖 in rust

> 以rust为例分享学习编程常考虑的方方面面
<!--ts-->
* [编程解剖 in rust](#编程解剖-in-rust)
   * [本地运行](#本地运行)
   * [git lfs配置](#git-lfs配置)
   * [项目基础结构](#项目基础结构)
   * [用到的工具](#用到的工具)
      * [mdbook-checklist: 整理待办事项](#mdbook-checklist-整理待办事项)
      * [mdbook-pagetoc: 添加业内目录](#mdbook-pagetoc-添加业内目录)
      * [mdbook-admonish: 使用新的css文件](#mdbook-admonish-使用新的css文件)
         * [相关资源](#相关资源)
         * [特别语法](#特别语法)
            * [自定义标题](#自定义标题)
            * [内嵌代码](#内嵌代码)
            * [自定义样式](#自定义样式)
            * [可折叠](#可折叠)
         * [常用格式](#常用格式)
            * [note](#note)
            * [abstract, summary, tldr](#abstract-summary-tldr)
            * [info, todo](#info-todo)
            * [tip, hint, important](#tip-hint-important)
            * [success, check, done](#success-check-done)
            * [question, help, faq](#question-help-faq)
            * [warning, caution, attention](#warning-caution-attention)
            * [failure, fail, missing](#failure-fail-missing)
            * [danger, error](#danger-error)
            * [bug](#bug)
            * [example](#example)
            * [quote, cite](#quote-cite)
   * [github action](#github-action)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: kuanhsiaokuo, at: Tue Jun 21 11:01:59 CST 2022 -->

<!--te-->

## 本地运行

```shell
cargo install mdbook
mdbook serve
```

## git lfs配置

- [Git Large File Storage | Git Large File Storage (LFS) replaces large files such as audio samples, videos, datasets, and graphics with text pointers inside Git, while storing the file contents on a remote server like GitHub.com or GitHub Enterprise.](https://git-lfs.github.com/)

```
git lfs install 
git lfs track '*.img'
```

## 项目基础结构

1. 每个文件夹下的同名md文件介绍当前文件夹的内容

## 用到的工具

> 来自：[Mdbook - Apple Power User](https://kuanhsiaokuo.github.io/apple_power_user/app_deepin/mdbook_deepin.html)

### mdbook-checklist: 整理待办事项

[ANSSI-FR/mdbook-checklist: mdbook preprocessor for generating checklists and indexes](https://github.com/ANSSI-FR/mdbook-checklist)

 ```shell
 cargo install mdbook-checklist
 ```

[mdbook-checklist - crates.io: Rust Package Registry](https://crates.io/crates/mdbook-checklist)

### mdbook-pagetoc: 添加业内目录

[JorelAli/mdBook-pagetoc: A page table of contents for mdBook](https://github.com/JorelAli/mdBook-pagetoc)

### mdbook-admonish: 使用新的css文件

#### 相关资源

- [tommilligan/mdbook-admonish: A preprocessor for mdbook to add Material Design admonishments.](https://github.com/tommilligan/mdbook-admonish)
- [mdbook-admonish - crates.io: Rust Package Registry](https://crates.io/crates/mdbook-admonish)
- [Overview - The mdbook-admonish book](https://tommilligan.github.io/mdbook-admonish/)
- [Admonitions - Material for MkDocs](https://squidfunk.github.io/mkdocs-material/reference/admonitions/#usage)

> All supported directives are listed below.

#### 特别语法

##### 自定义标题

```admonish warning title="数据损失"
The following steps can lead to irrecoverable data corruption.
```

##### 内嵌代码

~~~admonish bug title="内嵌代码"
This syntax won't work in Python 3:
```python
print "Hello, world!"
```
~~~

##### 自定义样式

```admonish note class="custom-0 custom-1"
Styled with my custom CSS class.
```

##### 可折叠

```admonish collapsible=true
Content will be hidden initially.
```

#### 常用格式

##### note

```admonish note
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `abstract`, `summary`, `tldr`

```admonish abstract
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `info`, `todo`

```admonish info
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `tip`, `hint`, `important`

```admonish tip
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `success`, `check`, `done`

```admonish success
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `question`, `help`, `faq`

```admonish question
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `warning`, `caution`, `attention`

```admonish warning
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `failure`, `fail`, `missing`

```admonish failure
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `danger`, `error`

```admonish danger
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `bug`

```admonish bug
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `example`

```admonish example
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

##### `quote`, `cite`

```admonish quote
Rust is a multi-paradigm, general-purpose programming language designed for performance and safety, especially safe concurrency.
```

## github action

- [rust-cargo-install · Actions · GitHub Marketplace](https://github.com/marketplace/actions/rust-cargo-install) 