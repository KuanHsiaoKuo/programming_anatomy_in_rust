# frame/executive语法点

> 静态->泛型->trait->trait bound->trait object: struct、enum和impl

<!-- ts -->

<!-- te -->

## 核心思路

- 所有权、借用、切片
- 泛型、trait、生命周期

### 从静态语言开始

每个方法都需要指定特定类型

### 为了复用，加入泛型

可以用泛型指定所有类型，就不需要重复写同样的方法

### 为了范围，加入接口/trait

其实写方法就是为了写特定操作，那么不如把操作作为重点，于是就有了接口

### 泛型+trait限定会导致编译的包膨胀，于是考虑trait对象

- [精通rust-4.6 使用trait对象实现真正多态性](marginnote3app://note/17559D64-980D-4EA7-B69E-49D766438399)

> 特征和泛型通过单态化(早期绑定)或运行时多态(后期绑定)
>
提供了两种代码复用的方式。何时使用它们取决于具体情况和相关应用程序的需求。通常,错误类型会被分配到动态分发的序列,因为它们应该是很少被执行的代码路径。单态化对小型的应用场景来说非常方便,但是缺点是导致了代码的膨胀和重复,这会影响缓存效率,并增加二进制文件的大小。但是,在这两个选项中,静态分发应该是首选,除非系统对二进制文件大小存在严格的限制。

### 为了隐私，加入pub

## 一、泛型-Generic Types

### 泛型由来

> 泛型本质上是对多类型的抽象，因为对于静态语言来说需要指明类型，对于那些很多类型通用的函数，不能一个个地写出来，那样编写、维护很麻烦。这个时候就需要用泛型了。

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

```

### 泛型使用

#### 范型结构体

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

#### 范型枚举体

```rust

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### 范型方法(impl)

> 注意，这里不是泛型函数。本质是因为方法是需要调用者，也就是某个对象；而函数是自己调用。

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

### 泛型的性能表现

- [Generic Data Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics)

## 区分一下类的组合与trait的组合

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

```

```rust
pub fn notify(item: &(impl Summary + Display)) {}
```

## 二、Trait

- [Traits: Defining Shared Behavior - The Rust Programming Language](https://doc.rust-lang.org/book/ch10-02-traits.html)

> A trait defines functionality a particular type has and can share with other types. We can use traits to define shared
> behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain
> behavior.

> Traits are similar to a feature often called interfaces in other languages, although with some differences.

### 关键字

#### 定义

trait

#### 实现

impl <trait name> for <struct name>{ fn <method_name in trait definition> }

#### 与实现方法的区别

Implementing a trait on a type is similar to implementing regular methods. The difference is that after impl, we put the
trait name we want to implement, then use the for keyword, and then specify the name of the type we want to implement
the trait for. Within the impl block, we put the method signatures that the trait definition has defined. Instead of
adding a semicolon after each signature, we use curly brackets and fill in the method body with the specific behavior
that we want the methods of the trait to have for the particular type.

### 举例

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}


```

### 限制：孤儿原则

we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within
our aggregator crate, because Display and Vec<T> are both defined in the standard library and aren’t local to our
aggregator crate. This restriction is part of a property called coherence (相干) , and more specifically the orphan (孤儿)
rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code
and vice (副) versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t
know which implementation to use.

### 常用方式

#### 在trait定义中编写默认方法逻辑

#### 作为函数入参:

##### &impl

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

#### 语法糖：trait限定(trait bounds)

##### 使用T代称

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```

##### 使用“+”组合继承

```rust
pub fn notify(item: &(impl Summary + Display)) {}
```

##### 使用where后置

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}

```

#### 作为函数出参

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

##### 条件出参

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

#### 泛型trait多态

> 根据泛型的trait设定不同函数

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

## 三、Trait Objects

- [Trait object types - The Rust Reference](https://doc.rust-lang.org/stable/reference/types/trait-object.html)

> A trait object is an opaque value of another type that implements a set of traits. The set of traits is made up of an
> object safe base trait plus any number of auto traits.

### 关键字

dyn Trait

> Before the 2021 edition, the dyn keyword may be omitted .

### 存在形式

```rust
dyn Trait
dyn Trait + Send
dyn Trait + Send + Sync
dyn Trait + 'static
dyn Trait + Send + 'static
dyn Trait +
dyn 'static + Trait.
dyn (Trait)
```

### 举例

```rust
trait Printable {
    fn stringify(&self) -> String;
}

impl Printable for i32 {
    fn stringify(&self) -> String { self.to_string() }
}

fn print(a: Box<dyn Printable>) {
    println!("{}", a.stringify());
}

fn main() {
    print(Box::new(10) as Box<dyn Printable>);
}
```

### 面向对象编程应用(trait objects + struct/enum)

- [Using Trait Objects That Allow for Values of Different Types - The Rust Programming Language](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

> We can use trait objects in place of a generic or concrete type.

Wherever we use a trait object, Rust’s type system will ensure at compile time that any value used in that context will
implement the trait object’s trait. Consequently, we don’t need to know all the possible types at compile time.

> We’ve mentioned that, in Rust, we refrain from calling structs and enums “objects” to distinguish them from other
> languages’ objects.


In a struct or enum, the data in the struct fields and the behavior in impl blocks are separated, whereas in other
languages, the data and behavior combined into one concept is often labeled an object.

However, trait objects are more like objects in other languages in the sense that they combine data and behavior.

But trait objects differ from traditional objects in that we can’t add data to a trait object. Trait objects aren’t as
generally useful as objects in other languages: their specific purpose is to allow abstraction across common behavior.

- trait objects

```rust
pub struct Screen {
    // This vector is of type Box<dyn Draw>
    // which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

- trait bounds

```rust
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

// If you’ll only ever have homogeneous collections, 
// using generics and trait bounds is preferable because 
// the definitions will be monomorphized at compile time to use the concrete types.
impl<T> Screen<T>
    where
        T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

- static or dynamic dispatch

1. trait限定：静态分发，编译期确定，对应实现了trait限定的类型
2. trait对象：动态分发，编译器指针，运行期确定具体类型

```rust
pub fn trait_object() {
    #[derive(Debug)]
    struct Foo;
    trait Bar {
        fn baz(&self);
    }
    impl Bar for Foo {
        fn baz(&self) { println!("{:?}", self) }
    }
    fn static_dispatch<T>(t: &T) where T: Bar {
        t.baz();
    }
    fn dynamic_dispatch(t: &Bar) {
        t.baz();
    }
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);
}

```

#### trait objects 其实就是鸭子类型

> 在编译期编译器就知道你的类型是否具有指定的动作，提前检查

This concept—of being concerned only with the messages a value responds to rather than the value’s concrete type—is
similar to the concept of duck typing in dynamically typed languages:

> if it walks like a duck and quacks like a duck, then it must be a duck!

In the implementation of run on Screen in Listing 17-5, run doesn’t need to know what the concrete type of each
component is.

It doesn’t check whether a component is an instance of a Button or a SelectBox, it just calls the draw method on the
component.

> By specifying Box<dyn Draw> as the type of the values in the components vector, we’ve defined Screen to need values
> that we can call the draw method on.

The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that:

- we never have to check whether a value implements a particular method at runtime or worry about getting errors if a
  value doesn’t implement a method but we call it anyway.
- Rust won’t compile our code if the values don’t implement the traits that the trait objects need.

## 四、trait object 与 trait bound的对比

- [Performance of Code Using Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics)
- [Trait Objects Perform Dynamic Dispatch](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch)

Recall in the “Performance of Code Using Generics” section in Chapter 10 our discussion on the monomorphization process
performed by the compiler when we use trait bounds on generics: the compiler generates nongeneric implementations of
functions and methods for each concrete type that we use in place of a generic type parameter. The code that results
from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile
time. This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re
calling. In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.

When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used
with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call. Instead, at
runtime, Rust uses the pointers inside the trait object to know which method to call. This lookup incurs a runtime cost
that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method’s
code, which in turn prevents some optimizations. However, we did get extra flexibility in the code that we wrote in
Listing 17-5 and were able to support in Listing 17-9, so it’s a trade-off to consider.

- tait对象编译期间无法确定对象大小，所以需要使用指针形式(引用)。

> trait作为参数一般有两种写法：

- 一种是trait对象，需要运行时才能获取对象大小属于动态分发，
- 一种是trait限定，类似模板是编译器件确定属于静态分发。

## 五、impl使用场景：实现方法，而非函数。

> impl与fn不会共存

### impl <> xxx<>结构体/枚举体：定义并实现方法

> impl<target types> <struct/enum><target types>

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

### impl<> for xxx 结构体/枚举体+trait：实现接口定义的方法

> impl <trait_name> for <struct/enum name>

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

## Rust面向对象

> Struct/Enum+Trait、Reusable、Encapsulation、Inheritance

- [Characteristics of Object-Oriented Languages - The Rust Programming Language](https://doc.rust-lang.org/book/ch17-01-what-is-oo.html)
- [Implementing an Object-Oriented Design Pattern - The Rust Programming Language](https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html)

> Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that
> data. The procedures are typically called methods or operations.

Using this definition, Rust is object-oriented: structs and enums have data, and impl blocks provide methods on structs
and enums. Even though structs and enums with methods aren’t called objects, they provide the same functionality,
according to the Gang of Four’s definition of objects.

### 注意：可以使用图表，对比python和rust的对应面向对象特性实现方式

#### 属性方法

#### 静态方法

#### 实例方法

#### 类方法

### Encapsulation

pub(crate/super) keywords

- [Modules Cheat Sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)

#### Start from the crate root

#### Declaring modules

#### Declaring submodules

mod keyword

#### Paths to code in modules

#### Private vs public

#### The use keyword

### Inheritance

> If a language must have inheritance to be an object-oriented language, then Rust is not one. There is no way to define
> a struct that inherits the parent struct’s fields and method implementations without using a macro.

You would choose inheritance for two main reasons.

#### Reuse

One is for reuse of code: you can implement particular behavior for one type, and inheritance enables you to reuse that
implementation for a different type. You can do this in a limited way in Rust code using default trait method
implementations

#### Polymorphism

The other reason to use inheritance relates to the type system: to enable a child type to be used in the same places as
the parent type. This is also called polymorphism (多态性) , which means that you can substitute multiple objects for each
other at runtime if they share certain characteristics.

> Polymorphism: code that can work with data of multiple types

Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what
those types must provide. This is sometimes called **bounded parametric polymorphism** .

## 六、回顾trait，联系上生命周期

- [rust权威指南-trait定义共享行为](marginnote3app://note/133A41C4-ADA0-4101-B280-BCD4D3DB8014)

借助于trait和trait约束,我们可以在使用泛型参数来消除重复代码的同时,向编译器指明自己希望泛型拥有的功能。而编译器则可以利用这些trait约束信息来确保代码中使用的具体类型提供了正确的行为。在动态语言中,尝试调用一个类型没有实现的方法会导致在运行时出现错误。但是,Rust将这些错误出现的时期转移到了编译期,并迫使我们在运行代码之前修复问题。我们无须编写那些用于在运行时检查行为的代码,因为这些工作已经在编译期完成了。这一机制在保留泛型灵活性的同时提升了代码的性能。

生命周期是另外一种你已经接触过的泛型。普通泛型可以确保类型拥有期望的行为,与之不同的是,生命周期能够确保引用在我们的使用过程中一直有效
