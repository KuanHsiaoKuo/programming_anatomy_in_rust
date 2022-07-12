// nested_imports.rs
// 使用{}嵌套导入
use std::sync::{Arc, Mutex, mpsc::{channel, Sender, Receiver}};

fn consume(_tx: Sender<()>, _rx: Receiver<()>) { } 

fn main() {
    let _ = Arc::new(Mutex::new(40));
    let (tx, rx) = channel();
    consume(tx, rx);
}
