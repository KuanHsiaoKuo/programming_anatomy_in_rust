# programming_anatomy_in_rust

> 以rust为例分享学习编程常考虑的方方面面
<!--ts-->
* [programming_anatomy_in_rust](#programming_anatomy_in_rust)
   * [本地运行](#本地运行)
   * [git lfs配置](#git-lfs配置)
   * [项目基础结构](#项目基础结构)
   * [用到的工具](#用到的工具)
   * [github action](#github-action)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Jun 15 08:01:17 UTC 2022 -->

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

1. mdbook-checklist: 整理待办事项
   [ANSSI-FR/mdbook-checklist: mdbook preprocessor for generating checklists and indexes](https://github.com/ANSSI-FR/mdbook-checklist)
    ```shell
    cargo install mdbook-checklist
    ```
   [mdbook-checklist - crates.io: Rust Package Registry](https://crates.io/crates/mdbook-checklist)
2. mdbook-pagetoc: 添加业内目录
   [JorelAli/mdBook-pagetoc: A page table of contents for mdBook](https://github.com/JorelAli/mdBook-pagetoc)
3. mdbook-admonish: 使用新的css文件
    - [tommilligan/mdbook-admonish: A preprocessor for mdbook to add Material Design admonishments.](https://github.com/tommilligan/mdbook-admonish)
    - [Overview - The mdbook-admonish book](https://tommilligan.github.io/mdbook-admonish/)
    - [Admonitions - Material for MkDocs](https://squidfunk.github.io/mkdocs-material/reference/admonitions/#usage)

## github action

- [rust-cargo-install · Actions · GitHub Marketplace](https://github.com/marketplace/actions/rust-cargo-install) 