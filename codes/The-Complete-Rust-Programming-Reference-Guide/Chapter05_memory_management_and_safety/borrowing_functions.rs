// borrowing_functions.rs

// 接收可变借用作为参数
fn take_the_n(n: &mut u8) {
    *n += 2;
}

fn take_the_s(s: &mut String) {
    s.push_str("ing");
}

fn main() {
    // 变量绑定必须是可变
    let mut n = 5;
    let mut s = String::from("Borrow");
    // 因为函数内做了修改，所以调用时也需要使用可变借用
    take_the_n(&mut n);
    take_the_s(&mut s);

    println!("n changed to {}", n);
    println!("s changed to {}", s);
}
