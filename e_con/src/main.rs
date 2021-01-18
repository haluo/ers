fn main() {
    //if
    let y  = 1;
    if y == 1 {
        println!("y = 1")
    }else if y == 2 {
        println!("y = 2")
    } else {
        println!("other")
    }

    //let中使用if
    let condition = true;
    let x = if condition{
        5
    }else{
        6
    };
    println!("x = {}",x);


    //loop
    let mut count = 0;
    loop{
        println!("in loop");
        if count ==10 {
            break;
        }
        count+=1;
    }
    let result = loop{
        count+= 1;
        if count == 20{
            break count*20;
        }
    };
    println!("result = {}",result);

    //while
    let mut i = 1;
    while i!=10 {
        i+=1;
    }
    println!("i = {}",i);

    //for
    let arr:[u32;5] = [1,2,3,4,5];
    for element in arr.iter(){
        println!("element = {}",element);
    }
    for element in &arr{
        println!("element = {}",element);
    }




    println!("Hello, world!");
}
