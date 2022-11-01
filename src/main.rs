fn main() {
    let mut x = 5;

    println!("number is {}", x);
    // 没有mut的话会提示不能对不可变变量进行二次赋值
    x = 6;

    println!("number is {}", x);
}
