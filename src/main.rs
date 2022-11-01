// 引入std的io模块
use std::io;
// rng是一个trait 它定义了随机数生成器所需要实现的方法集合
use rand::Rng;
// ordering是一个枚举类型
use std::cmp::Ordering;

fn main() {
    println!("guess the number");

    // rand::thread_rng会返回一个特定的随机数生成器
    // 它位于本地线程空间， 并通过操作系统获取随机数种子
    let secrect_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("please input your guess:");

        // mut关键词来申明一个变量是可控的
        // 创建一个新的String实例
        let mut guess = String::new();
        
        // read_line方法会将当前用户输入的数据不加区分的春初在字符串中
        // &意味着当前的蚕食是个引用。引用和变量一样，默认情况下也是不可变的。
        // readline会返回一个io::Result。Result会用用两个状态err和ok来返回结果。
        // expect会在err的状态下中断当前的程序，并将传入的字符串显现出来
        // ok则提取ok中附带的值并返回给用户
        io::stdin().read_line(&mut guess).expect("fail to read line");


        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // {}表示一个占位符
        println!("your guessed: {}", guess);

        // match表达式由数个分支组成，每个分支都包含用于匹配的模式以及匹配成功后要执行的代码
        match guess.cmp(&secrect_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }

    }

}
