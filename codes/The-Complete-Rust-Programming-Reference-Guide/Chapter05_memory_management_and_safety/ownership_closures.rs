// ownership_closures.rs

#[derive(Debug)]
struct Foo;

fn main() {
    let a = Foo;

    /*
        Foo 的所有权在闭包中已经默认移动到了 b,
        用户将无法再次访问 a。
    */
    // let closure = move || {
    let closure = || {
        let b = a;
    };

    println!("{:?}", a);
}
