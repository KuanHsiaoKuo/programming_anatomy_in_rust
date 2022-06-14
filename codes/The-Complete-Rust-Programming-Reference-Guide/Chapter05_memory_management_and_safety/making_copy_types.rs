// making_copy_types.rs

#[derive(Copy, Debug)]  // the trait `Clone` is not implemented for `Dummy`
struct Dummy;

fn main() {
    let a = Dummy;
    let b = a;
    println!("{:?}", a);
    println!("{:?}", b);
}
