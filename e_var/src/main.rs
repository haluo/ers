use std::u32;
//常量定义
const MAX_POINT:u32 = 10000;
fn main() {
    //变量定义  mut 为可变 没有则不可变
    let a = 1;
    println!("a = {}",a);
    let mut b: u32 = 2;
    b = 3;
    println!("b = {}",b);

    //隐藏性
    let b:f32 = 1.2;
    println!("b = {}",b);

    //常量
    println!("MAX_POINT = {}",MAX_POINT);
    
    println!("Hello, world!");
}
