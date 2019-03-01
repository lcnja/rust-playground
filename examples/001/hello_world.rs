// 单行注释

/*
    吧
    啦
    吧
    啦
    好
    几
    行
*/

// fn 声明 function 的关键字
fn main() {
    // 以 ! 结尾的都是宏
    // {} 格式字符串
    // {:?} Debug
    // {:02} 2 -> 02
    // {day} 命名参数
    // [详情参考](https://doc.rust-lang.org/std/fmt/)
    println!("{} {:?}, {}-{:02}-{day}", "Hello", "世界", 2019, 2, day = 29);
}

// 编译命令: rustc hello_world.rs
// 执行命令: ./hello_world
// 命令回显: Hello "世界", 2019-02-29

// 本代码中只埋了一个错误, 来debug玩吧.
