// trait_objects.rs

use std::fmt::Debug;

#[derive(Debug)]
struct Square(f32);

#[derive(Debug)]
struct Rectangle(f32, f32);

trait Area: Debug {
    fn get_area(&self) -> f32;
}

impl Area for Square {
    fn get_area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.0 * self.1
    }
}

fn main() {
    /*
        shapes 的元素类型是&dyn Area,这是一种表示为特征的类型。
        特征对象是由 dyn Area 表示的, 意味着它是指向 Area 特征某些实现的指针。
        特征对象形式的类型允许用户在集合类型(例如 Vec)中存储不同类型

        Square 和 Rectangle 会隐式转换成特征对象,因为我们给它们推送了一个引用。
        我们还可以通过手动转换某个特征对象来构造一个类型,但这是一种比较少见的情况.
        只有在编译器自身无法将类型作为特征对象转换时使用。
    */
    let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
    for s in shapes {
        println!("{:?}", s);
    }
}
