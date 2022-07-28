# 深入substrate runtime

<!--ts-->
* [深入substrate runtime](#深入substrate-runtime)
* [参考资源](#参考资源)
   * [Runtime总览](#runtime总览)
   * [Runtime与Smart Contracts](#runtime与smart-contracts)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Thu Jul 28 06:29:50 UTC 2022 -->

<!--te-->

# 参考资源

## Runtime总览

- [Runtime 总览 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/)

- [Runtime宏 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/macros)

```admonish tip title='学习 Substrate runtime宏的方法如下：'
阅读某个特定宏的文档
运行cargo expand命令去查看宏扩展后的代码
阅读宏定义。 如能很好地掌握 表达式模式匹配的宏规则是非常有帮助的。
```

- [Runtime 元数据 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/metadata)

> Runtim元数据就是用来RPC调用的。这里还说明了，各种语言如何通过RPC调用与链端交互

- [Runtime 存储 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/storage)
    1. Substrate 提供了分层的模块化存储API，使runtime开发人员能根据自身情况作出合适的存储决策。 但同时请记住，区块链runtime存储的基本原则是尽可能少的使用链上存储。
    2. [substrate/frame/support/src/storage at master · paritytech/substrate](https://github.com/paritytech/substrate/tree/master/frame/support/src/storage)

- [Runtime 来源 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/origin):
  Root\Signed\None

- [Runtime 执行流程 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/execution)
    1. [substrate/README.md at master · paritytech/substrate](https://github.com/paritytech/substrate/blob/master/frame/executive/README.md)
    2. 主要完成三种区块操作：初始化区块 执行Extrinsics 完结区块

- [Runtime事件 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/events)
    1. Substrate runtime模块通过触发事件，以向外部实体（例如用户，区块链链浏览器或dApps）通知runtime中的变化或状况。
    2. 模块所能触发的事件类型、事件中包含的信息，以及事件触发的时间都能被自定义。

- [Runtime 错误 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/errors)

- [链下工作机 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/off-chain-workers)

- [调试 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/debugging)

- [Runtime 测试 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/tests)

- [链上随机生成 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/randomness)
    1. [substrate/randomness.rs at master · paritytech/substrate](https://github.com/paritytech/substrate/blob/master/frame/support/src/traits/randomness.rs)

- [Runtime 升级 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/upgrades)

```admonish info title='两种升级方式都是累加，扩展，真正的更新还需要存储迁移'
在用于 runtime 开发的 FRAME 系统中，System 库定义了 set_code 函数 来更新 runtime 的定义。 

在 升级一条链 的教程里详细介绍了 FRAME runtime 的升级过程，并演示了两种不同的升级机制。 

该教程演示的两种升级方法严格意义上都是 累加型 的，这意味着它们通过 扩展，

而不是 更新 现有的 runtime 状态来修改 runtime 逻辑。 

如果 runtime 升级时对现时的状态有所更改，则可能有必要执行 “存储迁移”。

 
```

## Runtime与Smart Contracts

- [总览 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/smart-contracts/overview)
    1. Substrate Runtime 开发和 Substrate 智能合约是使用 Substrate 框架来构建 "去中心化应用" 的两种不同途径。