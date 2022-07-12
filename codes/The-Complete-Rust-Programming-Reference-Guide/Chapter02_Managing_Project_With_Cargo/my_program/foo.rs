
mod bar; // 使用同名目录foo下的文件模块foo/bar.rs

pub use self::bar::Bar; // 再次指定导入bar::Bar

pub fn do_foo() {
    println!("Hi from foo!");
}
