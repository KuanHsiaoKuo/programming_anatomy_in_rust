// ownership_match.rs

#[derive(Debug)]
enum Food {
    Cake,
    Pizza,
    Salad
}

#[derive(Debug)]
struct Bag {
    food: Food
}

fn main() {
    let bag = Bag { food: Food::Cake };
    match bag.food {
        Food::Cake => println!("I got cake"),
        // 以 ref 作为前缀。
        // 关键字 ref 可以通过引用来匹配元素,而不是根据值来捕获它们。
        ref a => println!("I got {:?}", a)
    }
    
    println!("{:?}", bag);
}
