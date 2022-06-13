// trait_bounds_functions.rs

use std::fmt::Debug;

trait Eatable {
    fn eat(&self);
}

// 指定类型必须是 Debug,以便其可以在方法内部输出到控制台
#[derive(Debug)]
struct Food<T>(T);

#[derive(Debug)]
struct Apple;

// 为了让 apple 是“可食用”的,我们实现了 Food 的 Eatable 特征
impl<T> Eatable for Food<T>
    where
        T: Debug,
{
    fn eat(&self) {
        println!("Eating {:?}", self);
    }
}

// 注意 eat 的特点,类型 T 必须实现 Eatable 特征。
fn eat<T>(val: T)
    where
        T: Eatable,
{
    val.eat();
}

fn main() {
    let apple = Food(Apple);
    eat(apple);
}
