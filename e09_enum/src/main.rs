//枚举经典写法
enum Message{
    Quit,//退出
    Move(u32,u32),//元组
    Change{a:u32,b:u32,c:u32},//结构体
    Send(String),//字符串
}
impl Message{
    fn print(&self){
        match self{ //*self 区别？
            Message::Quit => println!("im quit"),
            Message::Move(x,y) =>println!("move x={} y={}",x,y),
            Message::Change{a,b,c}=>println!("change a = {},b = {}, c = {}",a,b,c),
            Message::Send(s)=>println!("send: {}",s),
            _=>println!("default"),
        }
    }
}
fn main() {
    //枚举调用方法
    let mo = Message::Move(2,3);
    mo.print();
    let send = Message::Send(String::from("message body"));
    send.print();
    let change = Message::Change{a:1,b:2,c:3};
    change.print();
}