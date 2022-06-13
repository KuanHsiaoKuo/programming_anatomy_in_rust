// impl_trait_both.rs

use std::fmt::Display;

/*
1. 会接收任何 Display 特征的参数
2. 返回的类型是 impl Display
*/
fn surround_with_braces(val: impl Display) -> impl Display {
    format!("{{{}}}", val)
}

fn main() {
    println!("{}", surround_with_braces("Hello"));
}
