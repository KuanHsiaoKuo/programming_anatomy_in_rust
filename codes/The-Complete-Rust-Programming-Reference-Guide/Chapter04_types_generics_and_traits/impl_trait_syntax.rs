// impl_trait_syntax.rs

use std::fmt::Display;

/*
直接使用了 impl Display,而不是指定 T:Display。这是 impl 特征语法。
这为我们返回复杂或不方便表示的类型(例如函数的闭包)提供了便利
*/
fn show_me(val: impl Display) {
    println!("{}", val);
}

fn main() {
    show_me("Trait bounds are awesome");
}
