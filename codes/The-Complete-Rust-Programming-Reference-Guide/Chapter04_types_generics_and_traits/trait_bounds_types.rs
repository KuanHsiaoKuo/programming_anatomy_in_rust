// trait_bounds_types.rs

use std::fmt::Display;

// 写法一：区间内泛型
struct Foo<T: Display> {
    bar: T,
}

// 写法二：where语句解耦
struct Bar<F>
    where
        F: Display,
{
    inner: F,
}

fn main() {}
