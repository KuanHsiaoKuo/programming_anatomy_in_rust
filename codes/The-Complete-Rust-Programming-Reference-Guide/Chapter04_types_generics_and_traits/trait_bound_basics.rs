// trait_bounds_basics.rs

// 我们有一个方法 add_thing,它可以添加任何类型 T。
fn add_thing<T>(fst: T, snd: T) {
    // 编译器向用户建议在 T 上添加特征区间 Add
    // help: consider restricting type parameter `T`
    let _ = fst + snd;
}

fn main() {
    add_thing(2, 2);
}
