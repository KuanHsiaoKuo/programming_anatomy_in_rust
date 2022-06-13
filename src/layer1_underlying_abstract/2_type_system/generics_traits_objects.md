# 泛型、特征及特征对象

![rust-traits-deep-dive](https://raw.githubusercontent.com/KuanHsiaoKuo/writing_materials/main/imgs/rust-traits-deep-dive.png)
<!--ts-->
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

#### 泛型枚举体

#### 泛型特征

#### 泛型方法

#### 泛型实现块

## 特征

## 特征区间：泛型+特征

## 参考资源

1. <精通rust(第二版)>-第四章：类型、泛型和特征
2. [Advanced Traits - The Rust Programming Language](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
3. [Rust traits: A deep dive - LogRocket Blog](https://blog.logrocket.com/rust-traits-a-deep-dive/)

4. 《代码之髓》- 第五章：函数