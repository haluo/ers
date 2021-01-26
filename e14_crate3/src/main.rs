mod modeA{
    #[derive(Debug)]
    pub struct  A{
        pub number:i32,
        name:String,
    }
    impl A{
        pub fn new_a()->A{
            A{
                number:1,
                name:String::from("A name"),
            }    
        }
        pub fn print_a(&self){
            println!("number:{},name:{}",self.number,self.name);
        }
    }

    pub mod modeB{
        pub fn printB(){
            println!("B");
        }
        pub mod modeC{
            pub fn printC(){
               println!("C");
               super::printB(); //调用父模块！！
            }
        }
    }
}

use modeA::A ;
fn main() {
    // let a = modeA::A::new_a();
    let a = A::new_a();
    a.print_a();
    println!("--------------");
    modeA::modeB::modeC::printC();
    println!("Hello, world!");
}
