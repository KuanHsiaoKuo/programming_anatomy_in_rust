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
            * [bug](#bug)
            * [example](#example)
   * [github action](#github-action)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Nov  3 06:45:28 UTC 2022 -->

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

{{#check <name> | <description>}}

1. 每个文件夹下的同名md文件介绍当前文件夹的内容
2. 关于待完成内容：主要基于mdbook-checklist插件

```shell
    - [待完成](checklist.md) 
```

3. 添加待完成锚点的格式

> check空格之后的内容不能有空格, 且只能为英文
> " | "之后的内容可以有空格，可以为中文

```shell
{{#check Note-1 | This is an important note}}
```

- checklist页面渲染效果：

```none
- <SUMMARY对应标题名>
    - [This is an important note](Note-1)
```

> 这种写法会自动在本地生成md文件：src/checklist.md, 但是不用管它，最后渲染还是以mdbook-checklist的内容为准

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
- [ekalinin/github-markdown-toc: Easy TOC creation for GitHub README.md](https://github.com/ekalinin/github-markdown-toc)

```admonish info title='基于github repository设置secrects token'
> [Access Token | Code Cookbook](https://michaelcurrin.github.io/code-cookbook/recipes/ci-cd/github-actions/tokens/access-token.html)
1. 设置token：[Personal access tokens](https://github.com/settings/tokens)
2. 给指定repository设置secret：repository -> settings -> secrets -> Actions
3. 新建一个名为**GH_MD_TOC**的repository secret，将第一步的token设置进去
4. 将这个secret设置为action环境变量

```