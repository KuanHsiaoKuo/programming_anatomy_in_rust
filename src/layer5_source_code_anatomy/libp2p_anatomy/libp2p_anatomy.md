# libp2p源码学习

<!--ts-->
* [libp2p源码学习](#libp2p源码学习)
   * [Specification](#specification)
   * [Implementations](#implementations)
      * [go](#go)
      * [js](#js)
      * [rust](#rust)
      * [python](#python)
      * [cpp](#cpp)
   * [Community Discussion](#community-discussion)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Sep 13 12:51:04 UTC 2022 -->

<!--te-->

<p align="center">
  <a href="https://libp2p.io"><img src="https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/white-bg-2.png" /></a>
</p>

<p align="center">
  <a href="http://protocol.ai"><img src="https://img.shields.io/badge/made%20by-Protocol%20Labs-blue.svg?style=flat-square" /></a>
  <a href="http://libp2p.io/"><img src="https://img.shields.io/badge/project-libp2p-yellow.svg?style=flat-square" /></a>
  <a href="https://matrix.to/#/#libp2p:matrix.org"><img src="https://img.shields.io/badge/matrix-%23libp2p%3Amatrix.org-blue.svg?style=flat-square" /></a>
</p>


libp2p is a networking stack and library modularized out of The IPFS Project, and bundled separately for other tools to
use.

libp2p is the product of a long, and arduous quest of understanding -- a deep dive into the internet's network stack,
and plentiful peer-to-peer protocols from the past. Building large scale peer-to-peer systems has been complex and
difficult in the last 15 years, and libp2p is a way to fix that. It is a "network stack" -- a protocol suite -- that
cleanly separates concerns, and enables sophisticated applications to only use the protocols they absolutely need,
without giving up interoperability and upgradeability. libp2p grew out of IPFS, but it is built so that lots of people
can use it, for lots of different projects.

Learn more about libp2p at [**libp2p.io**](https://libp2p.io) and [**docs.libp2p.io**](https://docs.libp2p.io).

## Specification

> 主要介绍p2p网络相关知识

- [Specs](https://github.com/libp2p/specs)

## Implementations

> libp2p目前有5种语言实现版本：go、js、rust、python、cpp
> 不过rust版本才是核心，Central repository for work on libp2p

### go

- [go-libp2p](https://github.com/libp2p/go-libp2p) in Go

### js

- [js-libp2p](https://github.com/libp2p/js-libp2p) in Javascript, for Node and the Browser

### rust

> Central repository for work on libp2p

- [rust-libp2p](https://github.com/libp2p/rust-libp2p) in Rust

### python

- [py-libp2p](https://github.com/libp2p/py-libp2p) in Python

### cpp

- [cpp-libp2p](https://github.com/libp2p/py-libp2p) in C++

## Community Discussion

Please visit our discussion forums at [**discuss.libp2p.io**](https://discuss.libp2p.io) to get help, ask questions
about the past, present, and future of libp2p, and more.
