fn main() {
    let mut s0  = String::new();
    s0.push_str("hello");
    println!("s0 = {}",s0);

    let s1 = String::from("a string");
    println!("s1= {}",s1);

    let s1 = "b string".to_string();
    println!("s1 = {}",s1);



    let mut s2 = String::from("hello");
    s2.push_str(",world");
    let ss = "!".to_string();
    s2.push_str(&ss);
    println!("s2 = {}",s2);

    let mut s2 = String::from("tea");
    s2.push('m');
    println!("s2 = {}",s2);


    let s1 = "hello".to_string();
    let s2 = String::from(",world");
    let s3 = s1+&s2;
    println!("s3 = {}",s3);

    let s41 = String::from("a");
    let s42 = String::from("b");
    let s43 = String::from("c");
    let s44 = format!("{}-{}-{}",s41,s42,s43);
    println!("s44= {}",s44);


    let s4 =String::from("你好");
    let s41 = &s4[0..3];
    println!("s41 = {}",s41);


    //chars
    for c in s4.chars(){
        println!("c = {}",c);
    }
    //bytes
    for b in s4.bytes(){
        println!("b = {}",b);
    }

    println!("Hello, world!");
}
