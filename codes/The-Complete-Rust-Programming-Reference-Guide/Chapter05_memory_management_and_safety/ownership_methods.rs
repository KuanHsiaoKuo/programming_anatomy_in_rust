// ownership_methods.rs

struct Item(u32);

impl Item {
    fn new() -> Self {
        Item(1024)
    }
    /*
        以 self 作为第 1 个参数的实例方法。
        在调用之后, 它将在方法内移动, 并在函数作用域结束时被释放。
        后续我们将不能再使用它
    */
    fn take_item(self) {
        // does nothing
    }
}

fn main() {
    let it = Item::new(); // move occurs because `it` has type `Item`, which does not implement the `Copy` trait
    it.take_item(); // `it` moved due to this method call
    println!("{}", it.0); // value borrowed here after move
}
