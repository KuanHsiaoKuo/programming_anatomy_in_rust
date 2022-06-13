// impl_trait_closure.rs
// 它接收两个数字,并返回将这两个数字相加的闭包
fn lazy_adder(a: u32, b: u32) -> impl Fn() -> u32 {
    move || a + b
}

fn main() {
    // 调用 lazy_adder,传入两个数字。
    // 这会在 lazy_adder 中创建一个闭包,但不会对其进行求值
    let add_later = lazy_adder(1024, 2048);
    println!("{:?}", add_later());
}
