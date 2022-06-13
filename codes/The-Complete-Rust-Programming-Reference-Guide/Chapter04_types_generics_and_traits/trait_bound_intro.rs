// trait_bounds_intro.rs

struct Game;

struct Enemy;

struct Hero;
/*
在 Game 类型上我们有一个泛型函数 load,它可以接收任何游戏实体,
并通过任意 T 调用 init()将其加载到我们的游戏世界中。
但是,这个示例无法通过编译
*/
impl Game {
    fn load<T>(&self, entity: T) {
        entity.init(); // method not found in `T`
    }
}

fn main() {
    let game = Game;
    game.load(Enemy);
    game.load(Hero);
}
