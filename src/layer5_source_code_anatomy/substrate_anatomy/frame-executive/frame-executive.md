# frame/executive解剖

<!--ts-->
* [frame/executive解剖](#frameexecutive解剖)
   * [类型定义](#类型定义)
   * [特殊结构体定义(PhantomData)](#特殊结构体定义phantomdata)
   * [定义并实现许多方法](#定义并实现许多方法)
   * [实现trait中定义的方法](#实现trait中定义的方法)

<!-- Created by https://github.com/ekalinin/github-markdown-toc -->
<!-- Added by: runner, at: Wed Sep 14 14:33:57 UTC 2022 -->

<!--te-->

## 类型定义


```rust, editable
{{#include ../../../../codes/substrate/executive/src/lib.rs:139:141}}
```

## 特殊结构体定义(PhantomData)


```rust, editable
{{#include ../../../../codes/substrate/executive/src/lib.rs:143:170}}
```

## 定义并实现许多方法

- [Executive in frame_executive - Rust](https://paritytech.github.io/substrate/master/frame_executive/struct.Executive.html)

> substrate/frame/executive/src/lib.rs

```rust, editable
{{#include ../../../../codes/substrate/executive/src/lib.rs:205:224}}
```

## 实现trait中定义的方法

- [Rust之PhantomData - 简书](https://www.jianshu.com/p/0d60c148c0c0)
- [`type` alias vs `use` - help - The Rust Programming Language Forum](https://users.rust-lang.org/t/type-alias-vs-use/7486)
- 类型转换：
    - [as类型转换](marginnote3app://note/EA8CFC57-E675-493B-ACBA-BE60163B32EC)
    - [From与Into](marginnote3app://note/C1F89C89-164E-4A63-9214-0E2E335DFD00)
    - [整理类型转换](marginnote3app://note/D27CC849-C8B6-477F-A76C-DFF423784062)

- where与impl语法的对比：
    - [where可以用于更复杂的情况, 如关联类型](marginnote3app://note/8974BCC4-5036-4051-913A-287D6C6A56A5)
    - [泛型关联类型(Generic Associated Types)只能使用where子句](marginnote3app://note/E1B86A91-9D49-4CC5-9344-CAEB316EAC41)
    - [0135-where - The Rust RFC Book](https://rust-lang.github.io/rfcs/0135-where.html)

> substrate/frame/executive/src/lib.rs

1. 下列trait限定的意思：
    1. 为Executive结构体实现ExecuteBlock这个trait的方法
    2. for Executive<...>：Executive本身是个结构体，用到了这些类型
    3. impl<...>：这些类型分别有哪些trait限定，要用到关联类型限定的，就放在where子句中
    4. where子句：主要先约束好关联类型Block::Extrinsic，给后面的使用
    5. 总结impl与where子句：这里将简单情况放在impl中，将复杂的关联类型限定放在where子句中。
2. 关于CheckedOf、CallOf、OriginOf这三个：
    1. impl中的UnsignedValidator用到CallOf， CallOf用到CheckedOf
    2. (问题)OriginOf的用途

```rust, editable
{{#include ../../../../codes/substrate/executive/src/lib.rs:172:203}}
```
