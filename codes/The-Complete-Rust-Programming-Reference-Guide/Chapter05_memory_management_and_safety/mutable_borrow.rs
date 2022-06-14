// mutable_borrow.rs

fn main() {
    // 可变借用需要可变变量，所以这里会报错，可以加上mut
    let a = String::from("Owned string");
    /*
        用&mut a 创建了一个该值的可变借用。
        这并没有将 a 移动到 b
        只是可变地对它借用。
    */
    let a_ref = &mut a;
    a_ref.push('!');
}
