// dyn_trait.rs

use std::fmt::Display;

fn show_me(item: &dyn Display) {
    println!("{}", item);
}

fn main() {
    show_me(&"Hello trait object");
}