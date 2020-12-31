enum Message{
    Quit,
    Move(u32,u32),
    Change{a:u32,b:u32,c:u32},
    Send(String),
}
impl Message{
    fn print(&self){
        match *self{
            Message::Quit => println!("im quit"),
            Message::Move(x,y) =>println!("move x={} y={}",x,y),
            _=>println!("default"),
        }
    }
}
fn main() {
    //enum
    let mo = Message::Move(2,3);
    mo.print();
    //enmu Option<T>{
    //     Some(T),
    //     None
    // }
    let some_number = Some(5);
    let some_string = Some(String::from("a string"));
    let some_op:Option<i32> = None;
    println!("Hello, world!");

    let res = plus_one(some_number);
    match res{
        Some(x) => println!("res = {}",x),
        None => (),
    }
    if let Some(x) = plus_one(None){
        println!("res = {}",x);
    }else{
        println!("res is None");
    }
    
}

fn plus_one(x:Option<i32>)->Option<i32>{
    match x{
        None => None,
        Some(x) => Some(x+1),
    }
}