// complex/src/lib.rs

use std::ops::Add;

/*
将#[derive(Default)] 属性实现为一个过程宏,可以自动实现它修饰的类型的特征。
此自动派生要求任何自定义类型的字段(例如结构体或枚举)本身必须实现 Default 特征。
使用它们继承特征仅适用于结构体、枚举及联合。
*/
#[derive(Default, Debug, PartialEq, Copy, Clone)]
struct Complex<T> {
    // Real part
    re: T,
    // Complex part
    im: T,
}

impl<T> Complex<T> {
    // new 函数实际上并不是一个特殊的构造函数(如果你只了解带有构造函数的语言),
    // 而是社区采用的一个常用名称(作为创建新类型实例的方法名) 。
    fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

// 来自 std::ops 模块的 Add 特征允许我们使用“+”运算符将两个复数相加
/*
1. impl<T: Add<T, Output=T>表示我们正在为泛型 T 实现 Add,其中 T 实现 Add<T, Output=T>。
2. <T, Output=T>部分表示 Add 特征的实现必须具有相同的输入和输出类型
3. Add for Complex<T>部分表示为 Complex<T>类型实现 Add 特征
4. T:Add 表示必须实现 Add 特征。如果没有实现,那么我们不能使用“+”运算符
*/
impl<T: Add<T, Output=T>> Add for Complex<T> {
    type Output = Complex<T>;
    // Add 特征提供的核心功能,是我们在两种实现类型之间使用“+”运算符时调用的方法。
    // 它是一个实例方法,通过值获取 self 并接收 rhs 作为参数,即特征定义中的 RHS。
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}

// 来自 std::convert 模块的 Into 和 From 特征使用户能够根据其他类型创建复数类型
/*
1. 如果我们可以从内置基元类型 (例如双元素元组) 构造 Complex 类型
   其中第 1 个元素是实部,第 2 个元素是虚部,将会很方便。
   我们可以通过实现 From 特征来达到此目的。
2. 此特征定义了一个 from 方法, 为我们提供了在类型之间进行转换的一般方法
3. 第一个<T>是泛型 T 的声明, 第二个和第三个<T>是泛型类型 T 的用途。 我们会根据(T,T) 类型创建它
*/
impl<T> From<(T, T)> for Complex<T> {
    /*
        当我们实现它时, 只需要用我们希望实现它的类型替换 T 并实现 from 方法,
        然后我们就可以在相关类型上调用该方法。
        这是一个将 Complex 值转换为双元素元组类型的实现, Rust本身就能识别它
    */
    fn from(value: (T, T)) -> Complex<T> {
        Complex { re: value.0, im: value.1 }
    }
}

use std::fmt::{Formatter, Display, Result};

// Display 特征能够输出人类可读版本的复数类型
impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // 为了让用户能够以数学符号的形式查看复数类型
        write!(f, "{} + {}i", self.re, self.im)
    }
}

// 一个简单的初始化测试用例。
#[cfg(test)]
mod tests {
    use crate::Complex;

    #[test]
    fn complex_basics() {
        let first = Complex::new(3, 5);
        let second: Complex<i32> = Complex::default();
        assert_eq!(first.re, 3);
        assert_eq!(first.im, 5);
        assert!(second.re == second.im);
    }

    #[test]
    fn complex_addition() {
        let a = Complex::new(1, -2);
        let b = Complex::default();
        let res = a + b;
        assert_eq!(res, a);
    }

    #[test]
    fn complex_from() {
        let a = (2345, 456);
        let complex = Complex::from(a);
        assert_eq!(complex.re, 2345);
        assert_eq!(complex.im, 456);
    }

    #[test]
    fn complex_display() {
        let my_imaginary = Complex::new(2345, 456);
        println!("{}", my_imaginary);
    }
}
// 最后使用cargo test -- --nocapture执行