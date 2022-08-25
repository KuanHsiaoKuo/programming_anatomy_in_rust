# Substrate FRAME深入

<!--ts-->
* [Substrate FRAME深入](#substrate-frame深入)
   * [FRAME是缩写](#frame是缩写)
* [参考资源](#参考资源)
   * [FRAME与Runtime](#frame与runtime)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Aug 19 03:48:01 UTC 2022 -->

<!--te-->

## FRAME是缩写

```admonish hot title='frame还是FRAME'
Framework for Runtime Aggregation of Modularized Entities (FRAME) 
是一组可简化 runtime 开发的模块（称为 pallets ）和支持库
```

# 参考资源

## FRAME与Runtime

- [FRAME · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/frame)

> 这里还有对FRAME主要pallets的介绍

```admonish tip title='厘清FRAME、RUNTIME和pallet的关系'

![frame-relate-architecture](kroki-excalidraw:../../../../../materials/frame_relate_architecture.excalidraw)

![frame-relate-architecture](kroki-excalidraw:../../../../../materials/FRAME-pallets-Runtime.excalidraw)

1. FRAME 与 Runtime

Framework for Runtime Aggregation of Modularized Entities (FRAME)
是一组可简化 runtime 开发的模块（称为 pallets ）和支持库。 

2. FRAME 与 Runtime

其中 pallets 指的是 FRAME 中那些单一功能模块，承载着特定业务逻辑。
3. FRAME 不止 Runtime

FRAME 提供了一些与 Substrate Primitives 交互的帮助模块，还有智能合约宏展开
而 Substrate Primitives 则提供了与核心客户端的交互接口。


![substrate frontier frame runtime contract](kroki-excalidraw:../../../../../materials/substrate-frontier-frame-runtime-contract.excalidraw)

(点击链接直达源码)
```

```admonish info title='Runtime 把所有 pallets 组件整合起来。 '
它可用于定义和配置 runtime 里所含的模块 ( pallet )，
使模块之间得以联动来实现最终 runtime 的整体功能。 
当 runtime 接收到外部调用消息时，它会通过 Executive 模块
来将这些调用分派给相应的 pallets 进行处理。
```
