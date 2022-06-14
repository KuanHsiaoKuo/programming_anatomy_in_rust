# 所有权三件套：所有权、借用与生命周期

![Ownership](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/Ownership.jpeg)

<!--ts-->
* [所有权三件套：所有权、借用与生命周期](#所有权三件套所有权借用与生命周期)
   * [综述](#综述)
   * [所有权](#所有权)
      * [资源所有者](#资源所有者)
      * [资源所有者负责释放](#资源所有者负责释放)
         * [动态语言靠GC在运行期处理](#动态语言靠gc在运行期处理)
         * [静态语言在编译期规定](#静态语言在编译期规定)
      * [Rust的所有权规则](#rust的所有权规则)
      * [揣摩所有权含义](#揣摩所有权含义)
      * [作用域：所有权考虑因素](#作用域所有权考虑因素)
         * [多作用域的情况分析](#多作用域的情况分析)
         * [作用域是推断所有权时的一个重要属性](#作用域是推断所有权时的一个重要属性)
      * [引入移动和复制语义](#引入移动和复制语义)
         * [语义是什么意思](#语义是什么意思)
         * [移动语义：变量访问或重新分配时默认](#移动语义变量访问或重新分配时默认)
         * [复制语义](#复制语义)
         * [使用Copy特征更改语义](#使用copy特征更改语义)
         * [Copy 特征依赖于Clone 特征](#copy-特征依赖于clone-特征)
      * [区别一下Copy与Clone trait](#区别一下copy与clone-trait)
         * [Copy](#copy)
         * [Clone](#clone)
      * [Copy与Clone的使用原则](#copy与clone的使用原则)
         * [何时在类型上实现 Copy](#何时在类型上实现-copy)
         * [何时在类型上实现 Clone。](#何时在类型上实现-clone)
      * [所有权使用场景](#所有权使用场景)
         * [let绑定示例](#let绑定示例)
         * [将参数传递给函数](#将参数传递给函数)
         * [match表达式](#match表达式)
         * [impl代码块](#impl代码块)
         * [闭包](#闭包)
   * [借用: 规避所有权规则限制](#借用-规避所有权规则限制)
      * [为何需要借用/引用](#为何需要借用引用)
      * [借用 or 引用？](#借用-or-引用)
      * [两种借用方式](#两种借用方式)
         * [不可变借用：&amp;](#不可变借用)
         * [可变借用：&amp;mut](#可变借用mut)
      * [借用规则](#借用规则)
      * [如果违反借用规则](#如果违反借用规则)
         * [函数中的借用](#函数中的借用)
         * [匹配中的借用](#匹配中的借用)
         * [从函数返回引用：](#从函数返回引用)
   * [生命周期：针对引用附加的信息](#生命周期针对引用附加的信息)
   * [参考资源](#参考资源)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Tue Jun 14 11:33:29 UTC 2022 -->

<!--te-->

## 综述

```text
┌───────────────────────────────────┐
│                                   │
│                                   │
│         .───────.       ┌──────┐  │
│       ,'         `.     │Borrow│  │
│     ,'             `.   └──────┘  │
│    ;   ┌─────────┐   :            │
│    │   │OwnerShip│   │            │
│    │   └─────────┘   │            │
│    :     ┌─────┐     ; ┌────────┐ │
│     ╲    │Scope│    ╱  │Lifetime│ │
│      `.  └─────┘  ,'   └────────┘ │
│        `.       ,'                │
│          `─────'                  │
│                                   │
│                                   │
└───────────────────────────────────┘
```

所有权、作用域、借用与生命周期是 Rust 的内存安全及其零成本抽象原则的核心。 它们让 Rust 能够在编译期检测程序中内存安全违规,在离开作用域时自动释放相关资源等情况。
所有权有点类似核心原则,而借用和生命周期是对语言类型系统的扩展。在代码的不同上下文中加强或有时放松所有权原则,可确保编译期内存管理正常运作。

## 所有权

### 资源所有者

程序中资源的真正所有者的概念因语言而异。这里的含义是通过资源, 主要包含下列内容：

1. 共同引用在堆或堆栈上保存值的任何变量
2. 或者是包含打开文件描述符、数据库连接套接字、网络套接字及类似内容的变量。

从它们存在到完成程序调用及其之后的时间,都会占用一些内存。

### 资源所有者负责释放

> 资源所有者的一个重要职责就是明智地释放它们使用的内存,因为如果无法在适当的位置和时间执行取消内存分配,就可能导致内存泄漏。

#### 动态语言靠GC在运行期处理

在使用 Python 等动态语言编程时,可以将多个所有者或别名添加到 list 对象中,从而使用执行该对象的众多变量之一添加或删除 list 中的项目。变量不需要关心如何释放对象使用过的内存,因为 GC
会处理这些事情,并且一旦指向对象的所有引用都消失,GC 就会释放相关的内存。

#### 静态语言在编译期规定

对于 C/C++/Golang之类的编译语言,在智能指针出现之前,程序库对代码使用完毕的相关资源 API 的调用方或者被调用方是否负责释放内存有明确的规定。存在这些规则是因为编译器不会在这些语言中强制限定所有权。在
C++中不使用智能指针仍然有可能出现问题。在C++中,存在多个变量指向堆上的某个值是完全没问题的(尽管我们不建议这么做) ,这就是所谓的别名。由于具有指向资源的多个指针或别名的灵活性,程序员会遇到各种各样的问题,其中之一就是
C++中的迭代器失效问题。

> 具体而言,当给定作用域中资源的其他不可变别名相对存在至少一个可变别名时,就会出现问题

### Rust的所有权规则

Rust 试图为程序中值的所有权设定适当的语义。Rust 的所有权规则遵循以下原则。

- 使用 let 语句创建值或资源,并将其分配给变量时,该变量将成为资源的所有者。
- 当值从一个变量重新分配给另一个变量时, 值的所有权将转移至另一个变量, 原来的变量将失效以便另作他用。
- 值和变量在其作用域的末尾会被清理、释放。

### 揣摩所有权含义

需要注意的是,Rust 中的值只有一个所有者,即创建它们的变量。其理念很简单,但是它的含义值得揣摩：

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/ownership_basics.rs:1:}}
```

每当我们将变量分配给某个其他变量或从变量读取数据时,Rust 会默认移动变量指向的值。所有权规则可以防止你通过多个访问点来修改值,这可能导致访问已被释放的变量,即使在单线程上下文中,使用允许多个值的可变别名的语言也是如此

### 作用域：所有权考虑因素

> 为了分析某个值何时超出作用域,所有权规则还会考虑变量的作用域

1. 在 Rust 的背景下, 所有权与作用域协同工作。
2. 因此,作用域只不过是变量和值存在的环境。你声明的每个变量都与作用域有关。
3. 代码中的作用域是由一对花括号表示的。无论何时使用块表达式都会创建一个作用域,即任何以花括号开头和结尾的表达式。
4. 此外,作用域支持互相嵌套,并且可以在子作用域中访问父作用域的元素,但反过来不行

#### 多作用域的情况分析

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/scopes.rs:1:}}
```

#### 作用域是推断所有权时的一个重要属性

> 作用域还会被用来推断后续介绍的借用和生命周期。

1. 当作用域结束时,拥有值的任何变量都会运行相关代码以取消分配该值,并且其自身在作用域之外是无效的。
2. 特别是对在堆上分配的值,drop 方法会被放在作用域结束标记}之前调用。
3. 这类似于在 C 语言中调用 free 函数,但这里是隐式的, 并且可以避免程序员忘记释放值。
4. drop 方法来自 Drop 特征,它是为 Rust 中大部分堆分配类型实现的,可以轻松地自动释放资源。

### 引入移动和复制语义

> 结合作用域判断一下下列代码是否正确

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/ownership_primitives.rs:1:}}
```

#### 语义是什么意思

以移动语义为例，在 Rust
中,变量绑定默认具有移动语义。但这究竟意味着什么?要理解这一点,我们需要考虑如何在程序中使用变量。我们创建值或资源并将它们分配给变量,以便在程序中可以方便地引用它们。这些变量是指向值所在内存地址的名称。现在,诸如读取、赋值、添加及将它们传递给函数等对变量的操作,在访问变量指向值的方式上可能具有不同的语义或含义。在静态类型语言中,这些语义大致分为移动语义和复制语义。

#### 移动语义：变量访问或重新分配时默认

> 通过变量访问或重新分配给变量时移动到接收项的值表示移动语义。

由于Rust 的仿射类型系统,它默认会采用移动语义。仿射类型系统的一个突出特点是值或资源只能使用一次,而 Rust 通过所有权规则展示此属性。

#### 复制语义

默认情况下,通过变量分配或访问,以及从函数返回时复制的值(例如按位复制)具有复制语义。这意味着该值可以使用任意次数,每个值都是全新的。

#### 使用Copy特征更改语义

Rust 中的移动语义有时会受到限制。幸运的是,通过实现 Copy 特征可以更改类型的行为以遵循复制语义。基元和其他仅适用于堆栈的数据类型在默认情况下实现了上述特征,这也是前面的基元代码能够正常工作的原因

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/making_copy_types.rs:1:}}
```

#### Copy 特征依赖于Clone 特征

> Clone 是 Copy 的父级特征, 任何实现 Copy 特征的类型必须实现 Clone。

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/making_copy_types_fixed.rs:1:}}
```

### 区别一下Copy与Clone trait

> Copy 和 Clone 特征传达了在代码中使用类型时如何进行复制的原理。

| 特征  |     复制方式     | 复制内容 |               使用场景               |
| :---: | :--------------: | :------: | :----------------------------------: |
| Copy  | 隐式，自动化特征 |   堆栈   |     可以在堆栈上单独表示的小型值     |
| Clone |  显式调用clone   |  堆+栈   | 在堆上还包含一个值作为其表示的一部分 |

#### Copy

1. Copy 特征通常用于可以在堆栈上完全表示的类型, 也就是说它们自身没有任何部分位于堆上。
2. 如果出现了这种情况,那么 Copy 将是开销很大的操作,因为它必须从堆中复制值。这直接影响到赋值运算符的工作方式。
3. 如果类型实现了 Copy,则从一个变量到另一个变量的赋值操作将隐式复制数据。

#### Clone

Clone 特征用于显式复制, 并附带 clone 方法, 类型可以实现该方法以获取自身的副本

Clone 有一个名为 clone 的方法,用于获取接收者的不可变引用,即&self,并返回相同类型的新值。用户自定义类型或任何需要提供能够复制自身的包装器类型,应通过实现clone 方法来实现 Clone 特征

> 一个通过 Clone 特征复制类型的示例

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/explicit_copy.rs:1:}}
```

### Copy与Clone的使用原则

#### 何时在类型上实现 Copy

> 可以在堆栈上单独表示的小型值如下所示。

- 如果类型仅依赖于在其上实现了 Copy 特征的其他类型, 则 Copy 特征是为其隐式实现的。
- Copy 特征隐式影响赋值运算符的工作方式。 使用 Copy 特征构建自定义外部可见类型需要考虑它是否会对赋值运算符产生影响。 如果在开发的早期阶段,
  你的类型是Copy,后续将它移除之后则会影响使用该类型进行赋值的所有环节。你可以通过这种方式轻松地破坏 API。

#### 何时在类型上实现 Clone。

- Clone 特征只是声明一个 clone 方法,需要被显式调用。
- 如果你的类型在堆上还包含一个值作为其表示的一部分, 那么可选择实现 Clone 特征,这也需要向复制堆数据的用户明确表示。
- 如果要实现智能指针类型(例如引用计数类型) ,那么应该在类型上实现 Clone 特征,以便仅复制堆栈上的指针。

### 所有权使用场景

> 重要的是我们能够识别它和编译器给出的错误提示信息

#### let绑定示例

#### 将参数传递给函数

> 如果将参数传递给函数,那么相同的所有权规则也同样有效

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/ownership_functions.rs:1:}}
```

> 如果我们只需要变量 s 的读取访问权限,那么可以让该代码正常工作的另一种方法是将字符串 s 传递回 main 函数

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/ownership_functions_back.rs:1:}}
```

#### match表达式

> 在 match 表达式中,移动类型默认也会被移动

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/ownership_match.rs:1:}}
```

#### impl代码块

> impl 代码块中,任何以 self 作为第一个参数的方法都将获取调用该方法的值的所有权。这意味着对值调用方法后,你无法再次使用该值

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/ownership_methods.rs:1:}}
```

#### 闭包

> 闭包接收不同的值取决于在其内部使用变量的方式

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/ownership_closures.rs:1:}}
```

## 借用: 规避所有权规则限制

### 为何需要借用/引用

Rust的所有权规则非常严格, 因为它只允许我们使用类型一次。 如果函数只需要对值的读取访问权限,那么我们需要再次从函数返回值,或者在它传递给函数之前复制它。如果类型没有实现 Clone 特征,那么后者可能无法实现其目的。
复制类型看起来似乎很容易绕过所有权规则,但是由于 Clone 总是复制类型,可能会调用内存分配器 API,这是一种涉及系统调用,并且开销高昂的操作,因此它无法满足零成本抽象承诺的所有要点。 随着移动语义和所有权规则的实施,在 Rust
中编写程序很快就会变得困难重重。幸运的是,我们引入了借用和引用类型的概念,它们放宽了规则所施加的限制,但仍然能够在编译期确保兼容所有权规则。

借用的概念是规避所有权规则的限制。进行借用时,你不会获取值的所有权,而是根据需要提供数据。这是通过借用值,即获取值的引用来实现的。为了借用值,我们需要将运算符&放在变量之前,&表示指向变量的地址。

### 借用 or 引用？

### 两种借用方式

#### 不可变借用：&

> 当我们在类型之前使用运算符&时,就会创建一个不可变借用。

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/borrowing_basics.rs:1:}}
```

#### 可变借用：&mut

> 可以使用&mut 运算符对某个值进行可变借用。 通过可变借用, 你可以改变该值。

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/mutable_borrow.rs:1:}}
```

> 可变借用可以改变值,但是不能销毁该值,因为它不是所有者。
> 如果a 在借用它的代码行之前被销毁,则借用失效

(存疑🤨，待修正🤔️)

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/exclusive_borrow.rs:1:}}
```

### 借用规则

> 通过引用来维护单一的所有权语义。这些规则如下所示

1. 一个引用的生命周期可能不会超过其被引用的时间。 这是显而易见的, 因为如果它的生命周期超过其被借用的时间,那么它将指向一个垃圾值(被销毁的值) 。
2. 如果存在一个值的可变借用,那么不允许其他引用(可变借用或不可变借用)在该作用域下指向相同的值。可变借用是一种独占性借用。
3. 如果不存在指向某些东西的可变借用, 那么在该作用域下允许出现对同一值的任意数量的不可变借用

### 如果违反借用规则

#### 函数中的借用

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/borrowing_functions.rs:1:}}
```

#### 匹配中的借用

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/borrowing_match.rs:1:}}
```

#### 从函数返回引用：


```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter05_memory_management_and_safety/return_func_ref.rs:1:}}
```

## 生命周期：针对引用附加的信息

## 参考资源

- [Object lifetime and ownership](https://www.ditsing.com/object-lifetime-and-ownership/)
- [What is Ownership? - The Rust Programming Language](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- <精通rust(第二版)>-5.7 内存安全三原则
- [go没有虚拟机怎么运行gc的？ - 知乎](https://www.zhihu.com/question/58863427)