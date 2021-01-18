fn other_fun(){
    println!("this is other fun");
}
fn other_fun1(a:i32,b:u32){
    println!("a = {},b = {}",a,b);
}
fn other_fun2(a:i32,b:i32)->i32{
    return  a+b;
}
fn other_fun3(a:i32,b:i32)->i32{
    a+b
}
fn main() {
    other_fun();
    other_fun1(-1,2);
    
    let r = other_fun2(-1, -2);
    println!("r = {}",r);
    let r2 = other_fun3(-1, -2);
    println!("r2 = {}",r2);
    

    //语句是执行一些操作，但不返回值的指令
    // let y = 1; //语句，不返回值
    // let x = (let y = 1);//错误

    //表达式会计算返回一些值
    let y = {
        let x = 1;
        x +1
    };
    println!("y = {}",y);


    println!("Hello, world!");
}
