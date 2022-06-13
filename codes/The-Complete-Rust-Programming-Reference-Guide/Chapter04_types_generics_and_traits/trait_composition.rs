// traits_composition.rs

trait Eat {
    fn eat(&self) {
        println!("eat");
    }
}

trait Code {
    fn code(&self) {
        println!("code");
    }
}

trait Sleep {
    fn sleep(&self) {
        println!("sleep");
    }
}

// 创建了一个新的特征 Programmer,它由 3 个特征组合而成:Eat、Code、 Sleep。
// 通过这种方式, 我们对类型设置了约束:
// 因此如果类型 T 实现了 Programmer, 那么它必须实现上述所有特征
trait Programmer: Eat + Code + Sleep {
    fn animate(&self) {
        self.eat();
        self.code();
        self.sleep();
        println!("repeat!");
    }
}

struct Bob;

impl Programmer for Bob {}

impl Eat for Bob {}

impl Code for Bob {}

impl Sleep for Bob {}

fn main() {
    Bob.animate();
}
