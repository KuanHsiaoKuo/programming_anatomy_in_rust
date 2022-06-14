# 泛型、特征及特征对象

![rust-traits-deep-dive](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/rust-traits-deep-dive.png)
<!--ts-->

* [泛型、特征及特征对象](#泛型特征及特征对象)
    * [泛型](#泛型)
        * [从代码复用出发](#从代码复用出发)
            * [函数作用不足](#函数作用不足)
            * [静态类型语言还需要泛型来复用代码](#静态类型语言还需要泛型来复用代码)
        * [泛型本质上是一种单态化](#泛型本质上是一种单态化)
        * [泛型使用方式](#泛型使用方式)
            * [泛型函数](#泛型函数)
            * [泛型结构体](#泛型结构体)
            * [泛型枚举体](#泛型枚举体)
            * [泛型特征](#泛型特征)
            * [泛型方法](#泛型方法)
        * [impl: 泛型实现块](#impl-泛型实现块)
            * [泛型实现](#泛型实现)
            * [专门化泛型](#专门化泛型)
        * [指定类型进行实例化](#指定类型进行实例化)
            * [基于类型实例化推断](#基于类型实例化推断)
            * [泛型函数调用某些方法](#泛型函数调用某些方法)
            * [turbofish: ::&lt;&gt;](#turbofish-)
    * [特征](#特征)
        * [从多态和代码复用的角度来看: 接口、鸭子类型还是特征？](#从多态和代码复用的角度来看-接口鸭子类型还是特征)
            * [接口](#接口)
            * [鸭子类型](#鸭子类型)
            * [特征](#特征-1)
        * [特征到底是什么？](#特征到底是什么)
        * [特征的多种表现形式](#特征的多种表现形式)
            * [标记(特征)](#标记特征)
            * [简单(特征)](#简单特征)
            * [泛型(特征)](#泛型特征-1)
            * [关联类型(特征)](#关联类型特征)
            * [继承(特征)](#继承特征)
    * [特征区间：泛型+特征](#特征区间泛型特征)
        * [引出特征区间](#引出特征区间)
        * [代码单体化](#代码单体化)
        * [指定特征区间的四个方法](#指定特征区间的四个方法)
            * [区间内泛型: fn fn_name&lt;T: target_trait&gt;(val: T)](#区间内泛型-fn-fn_namet-target_traitval-t)
            * [where语句: 当第一种方法签名过长时使用](#where语句-当第一种方法签名过长时使用)
            * [使用"+"组合多个特征](#使用组合多个特征)
            * [使用impl特征语法: 闭包常用](#使用impl特征语法-闭包常用)
        * [特征区间的使用场景](#特征区间的使用场景)
            * [在类型上使用：不建议](#在类型上使用不建议)
            * [泛型函数+impl代码块](#泛型函数impl代码块)
    * [参考资源](#参考资源)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Mon Jun 13 16:17:48 UTC 2022 -->

<!--te-->

## 泛型

### 从代码复用出发

#### 函数作用不足

一直以来，函数的实现方式就是基于c语言的goto指令：

![image-20220613150721312](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220613150721312.png)

通过进一步强化，就得到函数的实现方式：

![image-20220613150804039](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/image-20220613150804039.png)

> 但是也就这样了，函数只能实现流程上的复用，不能实现类型上的复用。这一点其实在python、js这些动态类型语言上面就不存在这个问题。对于Rust这种静态类型语言，对函数入参类型要求十分严格，于是泛型就应运而生。

#### 静态类型语言还需要泛型来复用代码

> 泛型编程是一种仅适用于静态类型编程语言的技术。它首次出现在 ML 语言中,是一种静态类型的函数式语言。

像 Python 这样的动态语言采用的是简单类型 (duck typing) , 其中的 API 是根据它们可以做什么,而不是它们是什么来处理参数的,因此不依赖于泛型。

泛型是语言设计特性的一部分, 可以实现代码复用, 并遵循不重复自己的原则 (Don't Repeat Yourself,DRY) 。采用这种技术,你可以使用类型占位符来编写算法、函数、方法及类型, 并在这些类型上指定一个类型变量(
使用单个字母,通常是 T、K 或 V) ,告知编译器在任何代码中实例化它们时要填充的实际类型。这些类型被称为泛型或元素。单个字母(例如类型 T)被称为泛型参数。当你使用或实例化任何泛型元素时,它们会被替换成诸如 u32 这样的具体类型。

### 泛型本质上是一种单态化

每次将泛型元素与具体类型一起使用时,都会在编译时用类型变量 T 生成该代码的特定副本,并将其替换为具体类型。**这种在编译时生成包含具体类型的专用函数的过程被称为单态化,这是执行与多态函数相反的过程。**

### 泛型使用方式

> 在使用泛型时，应该多去考虑它与不同元素结合使用的场景背后的思维方式。泛型可以与结构体、枚举、函数、特征、方法及代码实现块。它们的一个共同特征是泛型的参数是由一对尖头括号分隔,并包含于其中。

#### 泛型函数

为了创建泛型函数,我们需要将泛型参数放在函数名之后和圆括号之前,如下所示:

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/generic_function.rs:1:12}}
```

#### 泛型结构体

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/generic_struct.rs:1:21}}
```

#### 泛型枚举体

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/generic_enum.rs:1:10}}
```

#### 泛型特征

#### 泛型方法

### impl: 泛型实现块

#### 泛型实现

> 当为任何泛型编写 impl 代码块时,都需要在使用它之前声明泛型参数。T 就像一个变量—— 一个类型变量,我们需要先声明它 impl代码块实际上意味着我们正在为所有类型 T 实现这些方法,它们会出现在 Container<T>中。这个 impl 代码块是一个泛型实现。 因此,生成的每个具体 Container 都将有这些方法。

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/generic_struct_impl.rs:1:15}}
```

#### 专门化泛型

> 在这里, 由于 u32 是作为具体类型存在的, 因此我们不需要 impl 之后的<T>, 这是 impl 代码块的另外一个特性,它允许你通过独立实现方法来专门化泛型。 现在,我们也可以通过将 T 替换为任何具体类型来为 Container<T>编写更具体的 impl 代码块。以下就是它的实例:

```rust, editable
impl Container<u32> { 
    fn sum(item: u32) -> Self {
        Container { item }
    } 
}
```

### 指定类型进行实例化

> 每当我们进行实例化时, 编译器需要在其类型签名中知道 T 的具体类型以便替换,这为其提供了将泛型代码单态化的类型信息。 而具体类型的确定主要有三种方式：

1. 大多数情况下,具体类型是基于类型的实例化推断.
2. 对泛型函数调用某些方法来接收具体类型。
3. 在极个别情况下, 我们需要通过使用 `turbofish (::<>)`运算符输入具体类型来替代泛型以便辅助编译器识别。

#### 基于类型实例化推断

这是最常见的方式，主要基于类型特征(trait)。

#### 泛型函数调用某些方法

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/using_generic_func.rs:1:}}
```

#### turbofish: ::<>

1. 如果没有任何类型特征，代码将无法编译：👇

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/creating_generic_vec.rs:1:}}
```

2. 这时可以用下列三种方式指定

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/using_generic_vec.rs:1:}}
```

## 特征

### 从多态和代码复用的角度来看: 接口、鸭子类型还是特征？

从多态和代码复用的角度来看, 在代码中将类型的共享行为和公共属性与其自身隔离通常是一个好主意,并且能拥有专属于自己的方法。在这样做时,我们允许不同类型通过通用属性互相关联,使我们能够为 API 编程,使其参数更通用或更具包容性。
> 这意味着我们可以接收具有这些通用属性的类型,而不仅限于某种特定类型。

#### 接口

类似 Java 和 C#的面向对象编程语言中,接口表达了相同的理念,我们可以在其中定义多种类型能够实现的共享行为。例如,我们可以使用单个 sort 函数接收实现 Comparable 或者 Comparator
接口的元素列表,而不是使用多个 sort 函数接收整数值列表,以及用其他函数接收字符串值列表。这使得我们可以将任何可比较(Comparable)的内容传递给 sort 函数。

#### 鸭子类型

而Python同样有明确的特性，被称为"鸭子类型".
> “当看到一只鸟走起来像鸭子、游泳起来像鸭子、叫起来也像鸭子，那么这只鸟就可以被称为鸭子。”

在鸭子类型中，关注点在于对象的行为，能做什么；而不是关注对象所属的类型。例如，在不使用鸭子类型的语言中，我们可以编写一个函数，它接受一个类型为“鸭子”的对象，并调用它的“走”和“叫”方法。在使用鸭子类型的语言中，这样的一个函数可以接受一个任意类型的对象，并调用它的“走”和“叫”方法。如果这些需要被调用的方法不存在，那么将引发一个运行时错误。任何拥有这样的正确的“走”和“叫”方法的对象都可被函数接受的这种行为引出了以上表述，这种决定类型的方式因此得名。

鸭子类型通常得益于“不”测试方法和函数中参数的类型，而是依赖文档、清晰的代码和测试来确保正确使用。

在常规类型中，我们能否在一个特定场景中使用某个对象取决于这个对象的类型，而在鸭子类型中，则取决于这个对象是否具有某种属性或者方法——即只要具备特定的属性或方法，能通过鸭子测试，就可以使用。

#### 特征

Rust也有一个类似且功能强大的结构,被称为特征。Rust中的特征以多种形式存在, 我们将介绍一些最常见的形式并了解一些与它们简单交互的方式。此外,当特征与泛型搭配使用时,可以限制传递到 API
的参数范围。我们将会对特征进行比较深入的了解。

### 特征到底是什么？

### 特征的多种表现形式

#### 标记(特征)

#### 简单(特征)

#### 泛型(特征)

#### 关联类型(特征)

#### 继承(特征)

### 一些常用内置特征

#### Debug

这个特征有助于在控制台上输出类型以便进行调试。在组合类型的情况下,类型将以类似 JSON 的格式输出,其中带有花括号和其他括号,如果类型是字符串,将会用引号标识。这适用于 Rust 中的大多数内置类型。

#### PartialEq 和 Eq

这些特征允许两个元素相互比较以验证是否相等

#### Copy和Clone

这些特征定义了类型的复制方式。 简而言之,当在任何自定义类型上自动派生时,这些特征允许用户从实例创建新的副本:

1. 可以在实现 Copy 时隐式创建
2. 也可以在实现 Clone 时通过调用 clone() 显式创建。

> 请注意,Copy 依赖于在类型上实现的 Clone 特征

#### Display

#### Add

#### Into 和 From

### 一个完整例子

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/complex/src/lib.rs:1:}}
```

## 特征区间：泛型+特征

### 引出特征区间

首先看一下如下代码：

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/trait_bound_intro.rs:1:}}
```

1. 因此, 任何类型为 T 的泛型函数都不能知道或默认假定 init()方法存在于 T 之上。
2. 如果确实如此,那么它根本不是泛型,并且它们只能接收具有 init()方法的类型。
3. **因此,有一种方法可以让编译器知道这一点,并约束 load 通过特征能够接收的类型集,这就需要用到特征区间**。

> 我们可以定义一个名为 Loadable 的特征,并在我们的 Enemy 和 Hero 类型上实现它。

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/trait_bound_intro_fixed.rs:1:}}
```

1. 注意, “:Loadable”部分表明了我们指定特征范围的方式。特征区间允许我们限制泛型 API 可以接收的参数范围。

### 代码单体化

1. 指定泛型元素上的绑定的特征类似于我们为变量指定类型的方式
2. 但是此处的变量是泛型 T,类型是某些特征。例如 **T:SomeTrait**。
3. 定义泛型函数时几乎总是会用到特征区间。
4. 如果定义的泛型函数中的 T 不包含任何特征区间,我们就不能通过任何方法调用,因 Rust 不知道给定方法实现的方式。
5. 它需要知道 T 是否具有某个 foo 方法,以便**将代码单体化**

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/trait_bound_basics.rs:1:}}
```

> 修正后

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/trait_bound_basics_fixed.rs:1:}}
```

### 指定特征区间的四个方法

#### 区间内泛型: fn fn_name<T: target_trait>(val: T)

```rust
// 指定特征区间的一种方法, 它会接收任何实现了 Display 特征的类型
fn show_me<T: Display>(val: T) {
    //可以使用{}格式化字符串，因为有Display特征区间
    printin!("{}", val);
}
```

1. 这是在泛型函数的类型签名的长度较短时声明特征区间的常见语法。
2. 在指定类型的特征区间时,此语法也有效

#### where语句: 当第一种方法签名过长时使用

```rust
pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err>
    where F: FromStr { ... }
```

> 注意“where F: FromStr”部分告诉我们 F 类型必须实现 FromStr 特征。where 语句将特征区间和函数签名解耦,并使其可读

#### 使用"+"组合多个特征

- 先看一下标准库中 HashMap 类型的 impl 代码块:

```rust
// HashMap 键类型的 K 必须实现 Hash 特征和 Eq 特征
impl<K: Hash + Eq, V> HashMap<K, V, RandomState>
{}
```

- 一个更加具体的例子

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/trait_composition.rs:1:}}
```

#### 使用impl特征语法: 闭包常用

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/impl_trait_syntax.rs:1:}}
```

> 如果没有这种语法,则必须使用 Box 智能指针类型将其放在指针后面返回,这涉及堆分配。
> 闭包的底层结构由实现了一系列特征的结构体组成。**Fn(T) -> U** 特征就是其中之一

闭包使用示例：

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/impl_trait_closure.rs:1:}}
```

还可以在入参和返回使用：

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/impl_trait_both.rs:1:}}
```

1. 通常建议将特征区间的 impl 特征语法用做函数的返回类型。
2. 在参数位置使用它意味着我们不能使用 turbofish 运算符。
3. 如果某些相关代码使用 turbofish 运算符来调用软件包中的某个方法,那么可能导致 API 不兼容。
4. 只有当我们没有可用的具体类型时才应该使用它, 就像闭包那样。

### 特征区间的使用场景

#### 在类型上使用：不建议

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/trait_bounds_types.rs:1:}}
```

> 不过,我们并不鼓励在类型上使用特征区间,因为它对类型自身施加了限制。
> 通常, 我们希望类型尽可能是泛型,从而允许我们使用任何类型创建实例,并使用函数或方法中的特征区间对其行为进行限制。

#### 泛型函数+impl代码块

```rust, editable
{{#include ../../../codes/The-Complete-Rust-Programming-Reference-Guide/Chapter04_types_generics_and_traits/trait_bound_functions.rs:1:}}
```

## 参考资源

1. <精通rust(第二版)>-第四章：类型、泛型和特征
2. [Advanced Traits - The Rust Programming Language](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
3. [Rust traits: A deep dive - LogRocket Blog](https://blog.logrocket.com/rust-traits-a-deep-dive/)
4. 《代码之髓》- 第五章：函数
5. [鸭子类型 - 维基百科，自由的百科全书](https://zh.m.wikipedia.org/zh-hans/%E9%B8%AD%E5%AD%90%E7%B1%BB%E5%9E%8B)