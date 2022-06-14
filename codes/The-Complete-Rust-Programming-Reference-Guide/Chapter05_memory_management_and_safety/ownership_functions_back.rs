// ownership_functions_back.rs

fn take_the_n(n: u8) {}

// 添加了一个返回类型
fn take_the_s(s: String) -> String {
    println!("inside function: {}", s);
    // 并将传递的字符串返回给调用者
    s
}

fn main() {
    let n = 5;
    let s = String::from("string");

    take_the_n(n);
    let s = take_the_s(s);

    println!("n is {}", n);
    println!("s is {}", s);
}
