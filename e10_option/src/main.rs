//Option 是标注库定义的一个枚举形式：
// enmu Option<T>{
//     Some(T),
//     None,
// }
fn main() {
    let some_number = Some(5);
    let some_string = Some(String::from("a string"));
    let number:Option<i32> = None;

    let x:i32 = 5;
    let y:Option<i32> = Some(5);
    let mut tmp = 0;
    match y{
        Some(i )=> {tmp = i},
        Nome => {tmp = 0},    
    }
    let sum = x +tmp;
    println!("sum = {}",sum);

    let reslut = plus_one(y);
    match reslut {
        Some(i) => println!("result = {}",i),
        None => println!("result is null"),
    }

    //if let
    if let Some(v) = plus_one(y){
        println!("v = {}",v);
    } 

    if let Some(v) = plus_one(y){
        println!("v = {}",v);
    } else {
        println!("v  is null");     
    }
}

fn  plus_one(x:Option<i32>)->Option<i32>{
    match x {
        None =>None,
        Some(i) =>Some(i+1),
    }
}
