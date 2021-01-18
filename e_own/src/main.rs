//rust 通过所有权来管理内存，编译器在编译时会根据所有权对内存进行检查
//堆和栈  编译时数据类型大小是固定的分配到栈上 不固定 则分配到堆上
//作用域 {} 变量作用域从定义开始到所在{}结尾结束
fn main() {
    let x = 1;
    {
        let y = 2;
        println!("x = {}",x);
        println!("y = {}",y);
        //分配在栈上
    }
    {
        let mut s1 = String::from("hello");
        s1.push_str(" world");
        println!("s1 = {}",s1);
        //字符串分配在堆上 离开作用作用域 调用drop释放内存

        let s2 = s1;
        println!("s2 = {}",s2);
        // println!("s1 = {}",s1);// 移动 move to s1  s1赋值给s2之后 s1无效 s2指向s1之前的字符串，内存只用释放一次
        

        //clone 深拷贝
        let s3 = s2.clone();
        println!("s2 = {}",s2);
        println!("s3 = {}",s3);
    }

    //copy 分配在栈上的数据直接复制 不存在move问题，
    //所有的整形 浮点型 字符 布尔 元组 都具有copy 特征 赋值后仍然可以继续使用
    let a = 1;
    let b = a;
    println!("a = {},b = {}",a,b);

    println!("------------------------------------");
    //函数所有权：
    let s = String::from("hello");
    take_ownership(s);
    // println!("s  = {}",s);//所有权 move 无法继续使用 并且已在函数结束时回收
    //如果想继续使用 可以在函数中返回值 take_ownership2;

    let i  = 5;
    make_copy(i);
    println!("i = {}",i);//copy性质 可继续使用

}

fn take_ownership(s:String){
    println!("s  = {}",s);
}
fn take_ownership2(s:String)->String{
    println!("s  = {}",s);
    s
}
fn make_copy(i:i32){
    println!("i = {}",i);
}
