// ownership_primitives.rs

fn main() {
    let foo = 4623;
    /*
        4623 的所有权不会从 foo 转移到 bar,但 bar 会获得4623 的单独副本。
        看起来基元类型在 Rust 中会被特殊对待,它们会被移动而不是复制。
        这意味着根据我们在 Rust 中使用的类型,存在不同的所有权语义,这将引入移动和复制语义的概念
    */
    let bar = foo;
    println!("{:?} {:?}", foo, bar); 
}