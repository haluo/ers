//HashMap<K,V>
use std::collections::HashMap;
fn main() {
    let mut scores:HashMap<String,i32> = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 20);

    let keys = vec![String::from("Blue"),String::from("Red")];
    let values = vec![10,20];
    let scores:HashMap<_,_> = keys.iter().zip(values.iter()).collect();


    let key = String::from("Blue");
    let v = scores.get(&key);//return Option, use if let  or  match
    if let Some(x) = v {
        println!("x  = {}",x);
    }
    match v {
        Some(x)=>println!("x = {}",x),
        _=>println!("None"),
    }

    //遍历
    for (key,value) in &scores{
        println!("key = {},value = {}",key,value);
    }

    //直接插入
    let mut ss = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    ss.insert(String::from("one"), 10);
    println!("{:?}",ss);

    //不存在时添加
    let mut ss = HashMap::new();
    ss.insert(String::from("one"), 1);
    ss.insert(String::from("two"), 2);
    ss.insert(String::from("three"), 3);
    ss.entry(String::from("one")).or_insert(10);
    println!("{:?}",ss);


    //根据旧值更新
    let text = "hello world wonderful world";
    let mut map  = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("map = {:?}",map);



    println!("Hello, world!");
}
