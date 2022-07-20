# Substrate深入尝试pallet

<!--ts-->
* [Substrate深入尝试pallet](#substrate深入尝试pallet)
   * [文档/代码更新问题](#文档代码更新问题)
   * [Pallet前置Rust知识](#pallet前置rust知识)
   * [Pallet组成](#pallet组成)
   * [1. 设置昵称：添加第一个Pallet到Runtime](#1-设置昵称添加第一个pallet到runtime)
      * [runtime结构分析](#runtime结构分析)
      * [runtime/Cargo.toml结构分析](#runtimecargotoml结构分析)
         * [[package]{...}](#package)
         * [[package.metadata.docs.rs]{...}](#packagemetadatadocsrs)
         * [[dependencies]{...}](#dependencies)
         * [[build-dependencies]{...}](#build-dependencies)
         * [[features]{...}](#features)
      * [四步添加pallet](#四步添加pallet)
         * [添加依赖: Cargo.toml/[dependincies]](#添加依赖-cargotomldependincies)
         * [添加feature: Cargo.toml/[features]](#添加feature-cargotomlfeatures)
         * [配置-&gt;添加config接口: src/lib.rs](#配置-添加config接口-srclibrs)
         * [定义运行时: src/lib.rs/construct_runtime!](#定义运行时-srclibrsconstruct_runtime)
      * [编译-&gt;运行-&gt;启动前端](#编译-运行-启动前端)
      * [验证功能](#验证功能)
         * [为帐户设置昵称](#为帐户设置昵称)
         * [使用Nicks pallet查询账户信息](#使用nicks-pallet查询账户信息)
      * [可能出现的问题](#可能出现的问题)
   * [2. 指定调用源头unsigned, signed or sudo](#2-指定调用源头unsigned-signed-or-sudo)
      * [signed与sudo有不同权限。](#signed与sudo有不同权限)
   * [3. Pallet Hooks](#3-pallet-hooks)
   * [4. Pallet Extrinsics](#4-pallet-extrinsics)
   * [4. Pallet Errors](#4-pallet-errors)
   * [5. Pallet Config](#5-pallet-config)
   * [6. Pallet Use Other Pallet](#6-pallet-use-other-pallet)
   * [7. Pallet Extension](#7-pallet-extension)
   * [8. Pallet Debug](#8-pallet-debug)
   * [9. Pallet RPC](#9-pallet-rpc)
   * [10. Pallet Benchmarking](#10-pallet-benchmarking)
* [参考资料](#参考资料)
   * [pallet基础](#pallet基础)
      * [尝试添加pallet到runtime](#尝试添加pallet到runtime)
      * [编写pallet到rust前置知识](#编写pallet到rust前置知识)
      * [编写简单到pallet](#编写简单到pallet)
      * [pallet的组成](#pallet的组成)
   * [Pallet技巧细节](#pallet技巧细节)
      * [storage（链上）各个类型使用](#storage链上各个类型使用)
      * [Error类型的使用](#error类型的使用)
      * [写调度函数的套路](#写调度函数的套路)
      * [hooks的使用](#hooks的使用)
      * [pallet中的Config](#pallet中的config)
      * [在pallet中使用其它pallet](#在pallet中使用其它pallet)
      * [封装和扩展现有pallet](#封装和扩展现有pallet)
      * [调试](#调试)
      * [pallet中的类型转换；](#pallet中的类型转换)
      * [在pallet中使用链下工作者（Offchain worker）](#在pallet中使用链下工作者offchain-worker)
      * [在pallet中链上写本地存储（offchain index）；](#在pallet中链上写本地存储offchain-index)
      * [在pallet的ocw中使用链下存储（offchain storage）；](#在pallet的ocw中使用链下存储offchain-storage)
      * [在pallet中使用其它pallet（使用其它pallet的存储）；](#在pallet中使用其它pallet使用其它pallet的存储)
      * [在pallet中添加rpc接口](#在pallet中添加rpc接口)
      * [编写测试](#编写测试)
      * [benchmarking](#benchmarking)
      * [为某些trait提供默认实现。](#为某些trait提供默认实现)
      * [升级](#升级)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Jul 19 12:14:42 UTC 2022 -->

<!--te-->







## 3. Pallet Hooks

~~~admonish info title="基于执行过程看hooks"
```plantuml
{{#include ../../../../../materials/plantumls/substrate_activity_hooks.puml:1:}}
```
~~~

## 4. Pallet Extrinsics

{{#check Pallet-Extrinsics | pallet extrinsics 使用}}

## 4. Pallet Errors

{{#check Pallet-Errors | pallet errors 使用}}

## 5. Pallet Config

{{#check Pallet-Config | pallet config 使用}}

## 6. Pallet Use Other Pallet

{{#check Pallet-Use-Other-Pallet | pallet 使用其他 Pallet}}

## 7. Pallet Extension

{{#check Pallet-Extension | pallet 扩展 使用}}

## 8. Pallet Debug

{{#check Pallet-Debug | pallet 调试}}

## 9. Pallet RPC

{{#check Pallet-RPC | pallet rpc 使用}}

## 10. Pallet Benchmarking

{{#check Pallet-Benchmarking | pallet 基准测试}}

# 参考资料

