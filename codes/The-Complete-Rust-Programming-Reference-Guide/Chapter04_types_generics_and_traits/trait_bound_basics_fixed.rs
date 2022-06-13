// trait_bound_basics_fixed.rs

use std::ops::Add;

// 代码修改之后,我们将“:Add”添加到了 T 的后面,之后代码通过了编译
fn add_thing<T: Add>(fst: T, snd: T) {
    let _ = fst + snd;
}

fn main() {
    add_thing(2, 2);
}
