# 深入substrate runtime

<!--ts-->

* [深入substrate runtime](#深入substrate-runtime)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Jul 26 13:09:58 UTC 2022 -->

<!--te-->

# 参考资源

## Runtime总览

- [Runtime 总览 · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/)

## FRAME与Runtime

- [FRAME · Substrate Developer Hub](https://core.tetcoin.org/docs/zh-CN/knowledgebase/runtime/frame)

> Framework for Runtime Aggregation of Modularized Entities (FRAME) 是一组可简化 runtime 开发的模块（称为 pallets ）和支持库。 其中 pallets
> 指的是 FRAME 中那些单一功能模块，承载着特定业务逻辑。

![frame-relate-architecture](kroki-excalidraw:../../../../../materials/frame_relate_architecture.excalidraw)

```admonish info title='Runtime 库把所有 pallets 组件整合起来。 '
它可用于定义和配置 runtime 里所含的模块 ( pallet )，
使模块之间得以联动来实现最终 runtime 的整体功能。 
当 runtime 接收到外部调用消息时，它会通过 Executive 模块
来将这些调用分派给相应的 pallets 进行处理。
```
