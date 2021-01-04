//当调用一个函数，函数可能会失败，除了在函数中处理错误外，还可以将错误传给调用者，让调用者决定如何处理，这叫传播错误
//示例测试原型时候   panic/unwarp/expect
//实际项目中用Result：
//Option 、Result  前者用于值为空时  或者用于 错误时
use std::io;
use std::io::Read;
use std::fs::File;
fn main() {
    let r = read_username_from_file();
    match r {
        Ok(s) => println!("s = {}",s),
        Err(error)=>println!("err = {:?}",error)
    }
}

fn read_username_from_file()->Result<String,io::Error>{
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}


// fn read_username_from_file()->Result<String,io::Error>{
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }


// fn read_username_from_file()->Result<String,io::Error>{
//     let f = File::open("hello.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_)=>Ok(s),
//         Err(error)=>Err(error),
//     }
// }
