//Option 是标注库定义的一个枚举形式：
// enmu Option<T>{
//     Some(T),
//     None,
// }
fn main() {
    let some_number = Some(5);
    let some_string = Some(String::from("a string"));
    let number:Option<i32> = None;

    let x:i32 = 5;
    let y:Option<i32> = Some(5);

}
