fn main() {
    //定义结构体
    #[derive(Debug)]
    struct User{
        name:String,
        count:String,
        nonce:u64,
        active:bool,
    }
    //创建实例
    let xiaoming = User{
        name:String::from("xiaoming"),
        count:String::from("0001"),
        nonce:1000,
        active:true,
    };

    //修改字段
    let mut xiaohuang = User{
        name:String::from("xiaohuang"),
        count:String::from("0001"),
        nonce:1000,
        active:true,
    };
    xiaohuang.nonce = 20000;

    //参数名和字段名相同时简写
    let name = String::from("xiaoxiao");
    let count = String::from("0003");
    let nonce = 20000;
    let active = false;
    let user1 = User{name,count,nonce,active};

    //从其他结构体创建实例
    let user2 = User{
        nonce:9999,
        ..user1
    };
    println!("user2.name = {}",user2.name);
    println!("user2.nonce = {}",user2.nonce);

    println!("-----------------------");

    //元组结构体(字段没有名字，圆括号)
    struct Point(i32,i32);
    let a = Point(10,20);
    let b = Point(30,11);
    println!("a.0 = {},b.0 = {}",a.0,b.1);

    //没有任何字段的结构体
    struct A{};

    //打印结构体#[derive(Debug)]
    println!("xioaming = {:?}",xiaoming);
    println!("xioaming = {:#?}",xiaoming);
}
