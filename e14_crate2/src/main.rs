use mylib::factory::produce_a;
use mylib::factory::produce_b as b;
//use mylib::factory::*;
//!!! 注意配置文件增加依赖  mylib = {path="./mylib"}

fn main() {
    mylib::factory::produce_a::produce_a();
    produce_a::produce_a();
    b::produce_b();
    println!("Hello, world!");
}