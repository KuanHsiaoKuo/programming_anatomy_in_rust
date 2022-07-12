
pub struct Bar; // 导出给foo.rs使用

impl Bar {
    pub fn hello() {
        println!("Hello from Bar !");
    }
}
