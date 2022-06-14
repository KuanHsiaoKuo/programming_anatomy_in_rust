// making_copy_types.rs

// 可以在派生注释中的 Copy 旁边添加 Clone 特征来让该示例通过编译
#[derive(Copy, Clone, Debug)]
struct Dummy;

fn main() {
    let a = Dummy;
    let b = a;
    println!("{:?}", a);
    println!("{:?}", b);
}
