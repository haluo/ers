//Vec<T>
fn main() {
    let mut v:Vec<i32> = Vec::new();
    v.push(1);
    println!("Hello, world!");
    
    let v1 = vec![1,2,3];
    let one:&i32 = &v1[0];
    print!("one = {}",*one)
}
