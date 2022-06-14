// scopes.rs

/*
由于函数可以创建新的作用域, 因此 main 函数引入了根级别作用域 0,
在代码中定义为 level_0_str
*/
fn main() {
    let level_0_str = String::from("foo");
    /*
        在 0 级作用域中, 创建了一个新的作用域,即作用域 1,
        并且带有一个花括号,其中包含变量level_1_number。
    */
    {
        let level_1_number = 9;
        /*
            在 1 级作用域中, 创建了一个块表达式, 它成为 2 级作用域。
            在其中, 声明了另一个变量 level_2_vector,
            以便可以将 level_1_number 添加到其中,
            而level_1_number 来自其父级作用域 1
        */
        {
            let mut level_2_vector = vec![1, 2, 3];
            level_2_vector.push(level_1_number);    // can access
        } // level_2_vector goes out of scope here 

        level_2_vector.push(4);    // no longer exists
    } // level_1_number goes out of scope here
} // level_0_str goes out of scope here
