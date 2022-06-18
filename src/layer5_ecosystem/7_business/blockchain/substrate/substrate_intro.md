# Substrate介绍与源码解读

<!--ts-->

* [Substrate介绍与源码解读](#substrate介绍与源码解读)
    * [Gavin Wook、Polkadot and Substrate](#gavin-wookpolkadot-and-substrate)
        * [Gavin Wook与波卡跨链](#gavin-wook与波卡跨链)
        * [从波卡到Substrate](#从波卡到substrate)
        * [跨链的重要性](#跨链的重要性)
    * [项目模块](#项目模块)
    * [功能逻辑](#功能逻辑)
    * [特色代码](#特色代码)
    * [参考资源](#参考资源)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: kuanhsiaokuo, at: Sat Jun 18 22:57:49 CST 2022 -->

<!--te-->

## Gavin Wook、Polkadot and Substrate

### Gavin Wook与波卡跨链

所以对于开发人员来说，比起大家耳熟能详的v神(Vitalik Buterin)，更重要的人是实际上撑起整个以太坊世界的灵魂人物**Gavin Wood**。更何况很多山寨币实际上就是在以太坊的模型上修修改改，所以在我看来Gavin
Wood是撑起了当前半个区块链世界的人。

而Gavin Wood 离开了以太坊之后，开启的一个新项目叫做Polkadot(波卡)，这个项目的**目的就是跨链，为了把各个割裂的区块链孤岛能够联系一起**
。虽然目前有很多的项目都号称自己在做跨链，但是目前在我看来唯一在推进，逻辑上是可推理，之后可能成功的跨链项目就只有波卡能够成功。（后半段可能存在一定误导性，Polkadot的跨链可能在大部分人理解下应该是分片的变种，也就是基于Substrate开发的区块链可以部署到Polkadot上，经过Polkadot平台互相沟通）。

### 从波卡到Substrate

而Gavin Wood 在开发的波卡的过程中，经过不断地思考，认为其实区块链发展这几年，大家做的很多事情都是相同的。

```admonish abstract title='抽离共同点，开始造"轮子"'
那么在以往的软件开发中，当大家发现大家都在做相同的事情的时候，就会将这件事情进行抽象，然后造“轮子”，将这些高层次的东西做封装，成为“开发框架”，将背后复杂的基础设施都封装起来，而使用这个“框架”的开发人员，就可以更加专注于自己的业务逻辑，而不必花费大量的精力去造“轮子”去完成那些每个链都要做的事情。
```

Gavin Wood 在开发波卡的中途先暂停了波卡的开发，将波卡及以太坊已有的成果进行抽象，命名为substrate作为区块链开发的基础框架，并把全部精力都转移到了substrate开发中。

```admonish tip title='第一个轮子'
所以“substrate”就是区块链世界的第一个“轮子”。
```

### 跨链的重要性

另一方面，在软件开发领域或者互联网领域，大家其实都发现了占据了框架的地位实际上一定程度上占据了这个领域开发的生态，更何况对于跨链来说，当大家的链都比较同质化后，跨链会更加的方便。现在大家都把跨链当作区块链下一个引爆点，而跨链的属性界定了做“跨链”的人基本上只能一家独大，成为垄断地位。而接入跨链的链越多，这个跨链就越垄断(因为大家使用跨链就是为了在不同的链之间兑换代币，能换的代币越多，使用这个跨链的人就越多，生态就越集中)，

而Gavin Wood 提供的substrate框架又**能解决大部分链都在重复解决的问题**，所以大家就更倾向于使用substrate开发自己的链。

```admonish quote title='三角关系'
这样就会在“用户-链-跨链”这个三角关系中将三方的利益绑定在一起，互惠互利，共同进步。
```

## 项目模块

## 功能逻辑

## 特色代码

## 参考资源

- [paritytech/substrate: Substrate: The platform for blockchain innovators](https://github.com/paritytech/substrate)
- [链块与分散的数据 - 知乎](https://www.zhihu.com/column/c_74315572)
- [substrate 源码解析与运用 - 介绍 - 知乎](https://web.archive.org/web/20220618042220/https://zhuanlan.zhihu.com/p/47805322)