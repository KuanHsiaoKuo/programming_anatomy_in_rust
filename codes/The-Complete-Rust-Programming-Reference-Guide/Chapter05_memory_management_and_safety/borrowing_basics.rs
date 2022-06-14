// ownership_basics.rs

#[derive(Debug)]
struct Foo;

fn main() {
    let foo = Foo;
    /*
        注意变量 foo 之前的&。我们借用 foo 并将借用结果分配给 bar。
        bar 的类型为&Foo, 这是一种引用类型。
        作为一个不可变借用,我们不能通过 bar 改变 Foo 中的值
     */
    let bar = &foo;
    println!("Foo is {:?}", foo);
    println!("Bar is {:?}", bar);
}
