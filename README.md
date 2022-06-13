# programming_anatomy_in_rust
> 以rust为例分享学习编程常考虑的方方面面
<!--ts-->
* [programming_anatomy_in_rust](#programming_anatomy_in_rust)
   * [本地运行](#本地运行)
   * [git lfs配置](#git-lfs配置)
   * [项目基础结构](#项目基础结构)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Jun 13 07:37:57 UTC 2022 -->

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