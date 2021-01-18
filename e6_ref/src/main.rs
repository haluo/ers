fn gives_ownership()->String{
    let s = String::from("hello");
    s
}
fn take_and_gives_back(s:String)->String{
    s
}
fn main() {
    let s1 = gives_ownership();
    println!("s1 = {}",s1);

    let mut s2 = String::from("hello");
    s2 = take_and_gives_back(s2); //所有权move  需要重新赋值s2 才能继续使用
    println!("s2 = {}",s2);


    println!("----------------------------------------------");
    
    //引用(&)：：让我们创建一个指向值的引用，但不拥有它，所以当引用离开作用域后 也不会被drop
    //无法更改
    //借用(可变引用)&mut 可以修改
    
    
    let  mut s1 = String::from("hello");
    let len = calcute_length(&s1);//引用
    println!("s1 = {}",s1);
    println!("len = {}",len);
    modify_s(&mut s1); //可变引用
    println!("s1 = {}",s1);

    
    println!("----------------------------------------------");
    //可变引用后  之前的引用无法继续使用 
    let  mut s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;
    println!("r1 = {} r2 ={}",r1,r2);
    let r3 = &mut s1;
    // println!("r1 = {} r2 ={}",r1,r2); r1 r2 无法继续使用


    println!("----------------------------------------------");
    
}

fn calcute_length(s:&String)->usize{
    s.len()
}
fn modify_s(s:&mut String) {
    s.push_str(",world");
}