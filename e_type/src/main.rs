fn main() {
    //bool
    let is_true:bool = true;
    let is_false:bool = false;
    println!("is_true = {},is_false = {}",is_true,is_false);
    
    //char 32位 可存汉字
    let b1 = 'a';
    let b2 = '中';
    println!("b2 = {}",b2);

    //数字 i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
    let c1:i32 = 10;

    //自适应类型isize  usize
    println!("max = {}",usize::max_value());

    //数组 [type;size] size是类型一部分
    let arr:[u32;5] =[1,2,3,4,5]; 
    println!("arr[0] = {}",arr[0]);
    for i in &arr{
        println!("i  = {}",i);
    }
    //元组
    let tup:(i32,f32,char)=(-3,1.2,'好');
    println!("{}",tup.0);
    println!("{}",tup.1);
    println!("{}",tup.2);

    //元组拆解
    let (x,y,z) = tup;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);


    println!("Hello, world!");
}
