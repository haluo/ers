//可预期错误 Result<T,E>
//enmu Result<T,E>{
//  Ok(T),
//  Err(E),
// }
//异常bug panic
use std::fs::File;
fn main() {
    // panic!("crash here");
    
    
    // let f = File::open("hello.txt");
    // let r = match f {
    //     Ok(file)=>file,
    //     Err(error)=>panic!("err--- {}",error),
    // };

    //简写1
    // let f = File::open("hello.txt").unwrap();

    //简写2
    let f = File::open("hello.txt").expect("failed to open hello.txt");
}
