// trait_bounds_intro.rs

struct Game;

struct Enemy;

struct Hero;

trait Loadable {
    fn init(&self);
}
/*
我们分别为 Enemy 和 Hero 实现了 Loadable,还修改了 load 方法
*/
impl Loadable for Enemy {
    fn init(&self) {
        println!("Enemy loaded");
    }
}

impl Loadable for Hero {
    fn init(&self) {
        println!("Hero loaded");
    }
}

impl Game {
    /*
        我们必须在泛型声明旁边放置几个符号来指定特征,我们称之为特征区间
    */
    fn load<T: Loadable>(&self, entity: T) {
        entity.init();
    }
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}
