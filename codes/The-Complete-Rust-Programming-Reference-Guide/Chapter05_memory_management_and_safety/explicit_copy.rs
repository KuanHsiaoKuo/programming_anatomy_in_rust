// explicit_copy.rs

// 在 derive 属性中添加了一个 Clone 特征。
#[derive(Clone, Debug)]
struct Dummy {
    items: u32,
}

fn main() {
    let a = Dummy { items: 54 };
    // 有了Clone, 我们就可以在 a 上调用 clone 方法来获得它的新副本
    let b = a.clone();
    println!("a: {:?}, b: {:?}", a, b);
}
