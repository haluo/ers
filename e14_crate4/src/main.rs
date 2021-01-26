//!!配置文件新增依赖rust-crypto = "0.2"
extern crate crypto;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
fn main() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("hello world");
    let result = hasher.result_str();
    println!("res = {}",result);
    println!("Hello, world!");
}
