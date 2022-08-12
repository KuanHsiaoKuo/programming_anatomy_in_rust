<!--ts-->
* [类型系统起源](#类型系统起源)
   * [我们为什么需要在语言中使用类型?](#我们为什么需要在语言中使用类型)
      * [1. 从二进制数据格式到汇编](#1-从二进制数据格式到汇编)
      * [2. 从汇编到编程语言，类型系统应运而生](#2-从汇编到编程语言类型系统应运而生)
      * [3. 再来认真看看类型系统](#3-再来认真看看类型系统)
      * [4. 类型系统其实是一组规则](#4-类型系统其实是一组规则)
      * [5. 类型系统是对内存管理/安全的抽象](#5-类型系统是对内存管理安全的抽象)
         * [动态类型大小](#动态类型大小)
         * [类型布局](#类型布局)
         * [内部可变性](#内部可变性)
         * [子类型与协变](#子类型与协变)
         * [强制转换(type coercions)](#强制转换type-coercions)
   * [参考资料](#参考资料)
      * [online-book](#online-book)
      * [fragment](#fragment)
      * [local](#local)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Fri Aug 12 16:18:30 UTC 2022 -->

<!--te-->

# 类型系统起源

## 我们为什么需要在语言中使用类型?

> 这是一个很好的问题,可以作为理解编程语言类型系统的契机。

### 1. 从二进制数据格式到汇编

作为程序员,我们知道为计算机编写的程序在最底层是以 0 和 1
组成的二进制数据格式表示的。实际上,最早的计算机必须使用机器代码手动编程。最终,程序员意识到这样做非常容易出错,并且乏味、耗时。对大部分人来说,在二进制层面操作和推断这些实体是不切实际的。 到了 20 世纪 50 年代,
编程社区提出了机器代码助记符的概念, 这些助记符变成了今天我们熟知的汇编语言。

### 2. 从汇编到编程语言，类型系统应运而生

然后,编程语言应运而生,它们被编译成汇编代码,并允许编程人员编写人类可理解的代码,以方便计算机将其编译成机器代码。然而,大家平时所说的语言表达某些语义比较模糊,因此需要制定一套规则和条件,来表述用类似人类语言编写的计算机程序中可能或不可能存在的内容,即程序语义。这使得我们提出了类型和类型系统的理念。

### 3. 再来认真看看类型系统

类型是一组具名的可能值。例如,u8 是一种可能包含 0~255 的正数值类型。类型提供了一种方法来弥合我们创建的这些实体的底层表示与心理模型之间的差距。除此之外, 类型还为我们提供了**表示实体的意图、行为和约束的方法**：

> 它们定义了用户通过类型能够(不能够) 做什么。 例如, 它没有定义将字符串类型的值和数值类型的值相加的结果是什么。

### 4. 类型系统其实是一组规则

从类型来看,语言设计者构建了类型系统,这些系统是一组规则,用于管理不同类型在编程语言中的交互。它们可以用作推断程序的工具,并有助于确保程序能够正常运行并符合规范。类型系统根据其表达力进行限定,这仅表示你可以使用类型表达逻辑的程度,以及程序中的不变量。

> 例如 Haskell 是一种高级语言,它具有非常丰富的表现力的类型系统,而C 语言是一种低级语言,它只为我们提供了很少的基于类型的抽象。Rust 试图在这两个极端之间找到一种平衡。

### 5. 类型系统是对内存管理/安全的抽象

#### 动态类型大小

#### 类型布局

#### 内部可变性

#### 子类型与协变

#### 强制转换(type coercions)

## 参考资料

### online-book

- [Dynamically Sized Types - The Rust Reference](https://doc.rust-lang.org/stable/reference/dynamically-sized-types.html)
- [Type layout - The Rust Reference](https://doc.rust-lang.org/stable/reference/type-layout.html)
- [Interior mutability - The Rust Reference](https://doc.rust-lang.org/stable/reference/interior-mutability.html)
- [Subtyping and Variance - The Rust Reference](https://doc.rust-lang.org/stable/reference/subtyping.html)
- [Type coercions - The Rust Reference](https://doc.rust-lang.org/stable/reference/type-coercions.html)

### fragment

### local

- [<精通Rust(第二版)>-4.1 类型系统及其重要性](marginnote3app://note/7D37028B-C1FB-47BC-AA50-4EBA22EB9CC7)

