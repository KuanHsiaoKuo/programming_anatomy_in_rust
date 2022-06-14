// ownership_basics.rs

#[derive(Debug)]
struct Foo(u32);

fn main() {
    // 根据所有权规则, foo 是 Foo 实例的所有者
    let foo = Foo(2048);
    /*
        bar 成为Foo 实例的新所有者, 而旧的 foo 是一个废弃变量.
        经过此变动之后不能在其他任何地方使用
    */
    let bar = foo; // value moved here
    /*
        每当我们将变量分配给某个其他变量或从变量读取数据时,Rust 会默认移动变量指向的值。
        所有权规则可以防止你通过多个访问点来修改值,这可能导致访问已被释放的变量.
        即使在单线程上下文中,使用允许多个值的可变别名的语言也是如此
    */
    println!("Foo is {:?}", foo);  // value borrowed here after move
    println!("Bar is {:?}", bar);
}
