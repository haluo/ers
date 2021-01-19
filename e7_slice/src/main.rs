//1.字符串slice是String中一部分值的引用
//2.字面值就是slice
//3.其他类型的slice
fn main() {
    let s = String::from("hello world");
    let h = &s[0..5];
    println!("h = {}",h);
    let h = &s[0..=4];
    println!("h = {}",h);
    let h = &s[..=4];
    println!("h = {}",h);
    let h = &s[6..];
    println!("h = {}",h);
    let h = &s[..];
    println!("h = {}",h);
    // let ss = String::from("你好");
    // let  h = &ss[0..1];
    let s3 = "h";// &str 是不可变引用
    println!("-----------------------");


    let a = [1,2,3,4];
    let sss = &a[1..3];
    println!("sss[0] ={} ",sss[0]);
    println!("sss[1] ={} ",sss[1]);
    
}
