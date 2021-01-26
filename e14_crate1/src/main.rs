mod factory {
    pub mod produce_a{
        pub fn produce_a(){
            println!("produce a");
        }
    }
    mod produce_b{
        fn produce_b(){
            println!("produce b")
        }
    }
}

fn main() {
    factory::produce_a::produce_a();
    println!("Hello, world!");
}
