//Vec<T>
fn main() {
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    
    let v1 = vec![1,2,3];
    
    //下标获取
    let one:&i32 = &v1[0];
    println!("one = {}",*one);
    //match操作(推荐 不会越界)
    match v1.get(1){
        Some(value) => println!("value={}",value),
        _=>println!("None")
    }

    let mut v2:Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    //不可变遍历
    for i in &v2{
        println!("i = {}",i);
    }
    //可变遍历
    for i in &mut v2{
        *i+=1;
        println!("i = {}",i);
    }

    //通过枚举使Vec存储多种类型
    enum Context{
        Text(String),
        Float(f32),
        Int(i32),
    }
    let c = vec![Context::Text(String::from("a string")),Context::Int(-1)];
    
}
