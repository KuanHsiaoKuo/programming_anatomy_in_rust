// ownership_functions.rs

fn take_the_n(n: u8) {}

fn take_the_s(s: String) {}

fn main() {
    let n = 5;
    let s = String::from("string");
    // take_the_n 函数能够正常工作,是因为 u8(基元类型)实现了 Copy 特征
    take_the_n(n);
    /*
        String 并没有实现 Copy 特征,因此值的所有权在 take_the_s 函数中会发生移动。
        当函数返回时,相关值的作用域也随之结束,并且会在 s 上调用 drop 方法,
        这会释放 s 所使用的堆内存。
        因此,在函数调用结束后 s 将失效
        使用clone即可通过编译：take_the_s(s.clone())
    */
    take_the_s(s);

    println!("n is {}", n);
    println!("s is {}", s);
}
