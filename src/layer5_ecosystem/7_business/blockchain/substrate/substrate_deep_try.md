# Substrate深入尝试pallet

<!--ts-->
* [Substrate深入尝试pallet](#substrate深入尝试pallet)
   * [参考资源](#参考资源)
      * [pallet相关](#pallet相关)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: kuanhsiaokuo, at: Fri Jul  1 21:12:13 CST 2022 -->

<!--te-->

## 参考资源

### pallet相关

- [Pallet前置知识](https://web.archive.org/web/20220627101518/https://mp.weixin.qq.com/s/wPVbEeIVKdXGro0QYsmJBg)
    - trait的孤儿规则
    - trait对象
    - trait的继承
    - 关联类型
    - 定义Config trait，然后为Pallet实现相应的trait，最后在main函数中使用它
- [编写简单的pallet](https://web.archive.org/web/20220626145126/https://mp.weixin.qq.com/s/4vIelf3YSV4fybakkT6QPQ)
    - node-template的结构
    - 编写pallet的一般格式, 整理出7个部分, 1和2基本上是固定的写法，而对于后面的3-7部分，则是根据实际需要写或者不写。关于模板中每部分的解释，可以参考文档.
        1. 依赖;
        2. pallet类型声明;
        3. config trait;
        4. 定义要使用的链上存储;
        5. 事件;
        6. 钩子函数;
        7. 交易调用函数;
    - 举例编写simple-pallet
      > 功能介绍: simple-pallet是一个存证的pallet，简单说就是提供一个存取一段hash到链上的功能，和从链上读取的功能。
    - 将pallet添加到runtime中
    - 编译运行
    - 调试使用pallet中的功能
- [Pallet的组成](https://web.archive.org/web/20220627101333/https://mp.weixin.qq.com/s/1M2HBpxIDVPDwHvbTLEk4w)
  > 要想熟练的开发pallet，我们必须得把pallet中的各个组成部分弄清楚。本节，我们就按照模板中的各个部分的顺序来讲解pallet的组成
    1. 导出和依赖：Pub mod pallet{}就是将我们的pallet暴露出来， pub use pallet::*;是可以使用pallet中的所有类型，函数，数据等
    2. pallet类型声明：它是一系列trait和方法的拥有者，实际的作用类似于占位符，这里举例rust程序
    3. config trait： 这部分是指定Runtime的配置trait，Pallet中使用的一些类型和常量在此trait中进行配置。
    4. Storage-定义要使用的链上存储： 存储（Storage）允许我们在链上存储数据，使用它存储的数据可以通过Runtime进行访问。substrate提供了四种存储方式，分别是：
        - Storage Value: 存储单个的值, 无键
        - Storage Map: 以map方式存储，单键，key-value
        - Storage Double Map: 以双键方式存储，(key1, key2)-value
        - Storage N Map: 以多键方式存储，(key1, key2, ..., keyn)-value
    5. Event-事件：当pallet需要把运行时上的更改或变化通知给外部主体时，就需要用到事件。事件是一个枚举类型
    6. hooks-钩子函数：钩子函数，是在区块链运行过程中希望固定执行的函数，例如我们希望在每个区块构建之前、之后的时候执行某些逻辑等，就可以把这些逻辑放在钩子函数中
    7. Extrinsic-调度函数，交易调用函数: Extrinsic则是**可以从runtime外部可以调用的函数，也是pallet对外提供的逻辑功能**。比如交易
- [hooks: pallet的🪝钩子函数使用](https://web.archive.org/web/20220628021501/https://mp.weixin.qq.com/s/tPyB9CuTVP2Y1DGgl_VPyQ)
    - substrate中的执行过程
        1. 初始化区块（Initializes the block）
        2. 执行区块（Executes extrinsics）
        3. 确认区块（ Finalizes the block）.
    - hooks介绍:
        1. on_finalize: 在区块 finalize 的时候调用。
        2. on_idle：区块finalize的时候调用，不过比on_finalize先调用。
        3. on_initialize：区块初始化的时候调用。
        4. on_runtime_upgrade：执行模块升级的时候调用。
        5. pre_upgrade：升级之前的检查。
        6. post_upgrade：升级之后的处理。
        7. offchain_worker：在一个 pallet 上实现此函数后可以在此函数中长时间的执行需要链下执行的功能。该函数会在每次区块导入的时候调用。后续我们讲ocw使用的时候就需要和这个函数打交道。
        8. integrity_test：运行集成测试。
    - 示例
    - [资料](https://docs.substrate.io/v3/concepts/execution/)
    - [substrate源码](https://paritytech.github.io/substrate/master/frame_support/traits/trait.Hooks.html)
- [substrate轻松学：写调度函数](https://mp.weixin.qq.com/s/Xnv5aNiLn-NoH6obouaONg)
  > 调度函数在substrate官方文档里面叫做Extrinsics（外部调用），详细的Extrinsics介绍可以参考这里.在substrate中共有三种Extrinsics，分别是Inherents、Signed transactions和Unsigned transactions。而在我们开发pallet的过程中，比较常用到的是后两种，所以我们这里也主要介绍后两种，对于Inherents有兴趣的小伙伴可以自己看官方文档研究下。
    - Signed transactions
    - Unsigned transactions
    - 通常写法：调度函数的位置->函数体的写法->权重->transactional
    - 示例
    -
  参考：[extrinsics](https://docs.substrate.io/v3/concepts/extrinsics/)&[weights-and-fees](https://docs.substrate.io/v3/runtime/weights-and-fees/)
- [pallet中Error类型的使用](https://web.archive.org/web/20220627112629/https://mp.weixin.qq.com/s/cNijF5h2Yn7R-K0ryoOJrA)
  > 在runtime代码执行时，代码必须是“非抛出的”，或者说不应该panic，应该是优雅的处理错误，所以在写pallet代码时，允许我们自定义错误类型，当错误发生时，可以返回我们定义的错误类型。这里的Error类型是指运行时在执行调度函数（也就是交易函数）时返回的错误。因为在调度函数执行时，返回的结果为DispatchResult类型，当执行结果错误时，返回DispatchError。
    - 错误类型的定义
    - 在函数中返回错误
    - 简单示例
- [pallet中的config](https://web.archive.org/web/20220627112755/https://mp.weixin.qq.com/s/JOaBn4bkda2LicV3Lyb4tw)
    - 好好理解rust中关于trait和关联类型相关的知识
    - pallet 简单示例: 介绍一个存储学生信息的pallet，其中存储逻辑写在extrinsic中
    - 在Config中定义配置类型：主要使用trait约束和关联类型改写
    - 在runtime中指定具体的类型
    - 构建、交互与调试
    - [参考资料](https://docs.substrate.io/v3/runtime/events-and-errors/)
- [在pallet中使用其他pallet](https://web.archive.org/web/20220627101725/https://mp.weixin.qq.com/s/z4fefNUb3avcae0htHpxgQ)
    - 在自己的pallet中使用其它的pallet主要有以下几种情况：
        1. 指定某个现成的pallet: 在pallet的config中定义类型，然后runtime中使用时指定这个类型为frame中指定某个现成的pallet；
        2. 指定某个自定义的pallet: 在pallet的config中定义类型，然后runtime中使用时指定这个类型为frame中指定某个自定义的pallet；
        3. 封装和扩展现有的 pallet 。
    - 在runtime中直接指定某个类型为其它的pallet
      > 这种方式比较常见的就是在pallet中定义currency类型，然后用指定currency类型为balances pallet。详细的可以看substrate中node中的使用，在pallet_assets中使用了pallet_balances，就是通过指定前者的currency类型为后者
        - [详情](https://github.com/paritytech/substrate/blob/master/bin/node/runtime/src/lib.rs#L1343)
    - pallet中使用其它pallet的storage
      > 自定义两个pallet，分别叫做pallet-use-other-pallet1和pallet-storage-provider，然后我们在前一个pallet中读取和存储后一个pallet
- [封装和扩展现有的pallet](https://web.archive.org/web/20220627113013/https://mp.weixin.qq.com/s/23wuRo4gj4oH-3EG74NnTA)
    - 这里使用substrate提供的contracts pallet，然后对其中的功能进行封装。
      > 在我们的封装中，将contracts pallet的call函数封装成sudo_call，即需要root权限才能调用。同时，我们在runtime中加载contracts时，去掉直接调用contracts函数的方式。
    - 整个方式我们分成两大步骤，如下：
        1. 编写extend-pallet;
        2. 在runtime配置extend- pallet 和contracts pallet。
    - [文档资料](https://www.shawntabrizi.com/substrate/extending-substrate-runtime-modules/)

- [调试pallet](https://web.archive.org/web/20220627113043/https://mp.weixin.qq.com/s/Ddu-CPgRz-U7uO4PkUnp2g)
    - 在pallet开发时主要有以下几种调试方式：
        1. logging uilities;
        2. printable trait;
        3. print函数;
        4. if_std.
    - 使用logging uilities
    - 使用 printable trait
    - 使用print函数
    - 使用 if_std
    - [文档资料](https://docs.substrate.io/v3/runtime/debugging/)
- [为pallet编写tests](https://web.archive.org/web/20220627101811/https://mp.weixin.qq.com/s/ZU5SYYrL6OORWGEbRev7Zg)
- [为pallet自定义rpc接口](https://web.archive.org/web/20220627101825/https://mp.weixin.qq.com/s/_QTUGTAWLreUVcNJcVKBjA)
  > pallet写好后需要通过runtime加载到链上（就是runtime/src/lib.rs中的construct_runtime宏包含的部分）。那么对应到我们的测试，如果对pallet进行测试，我们也需要构建一个runtime测试环境，然后在这个环境中加载pallet，对pallet进行测试。所以，编写pallet的测试就分为以下几部分：
    1. 编写 mock runtime;
    2. 编写pallet的genesisconfig;
    3. 编写测试。

    - [文档资料](https://docs.substrate.io/v3/runtime/custom-rpcs/)
- [为pallet编写benchmarking](https://web.archive.org/web/20220701080314/https://mp.weixin.qq.com/s/8DsbCwL8XkiIEbTlHx4oAg)
  > 为pallet编写benchmarking分两种情况，如下：
    1. 对函数进行性能测试时需要的构造条件不会涉及到本pallet以外的其它pallet；
    2. 在对函数进行性能测试时需要先使用其它的 pallet 构造测试的先决条件。
    ~~~admonish info title='大部分用第一种'
    - 第一种情况相对来说比较简单，这个也比较好找到例子。
    - 第二种情况则比较复杂，写起来也比较麻烦。
    - 不过在我们的开发中，大部分都是第一种情况。
    ~~~