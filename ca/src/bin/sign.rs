use std::env;
use ca::sign;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("引数には署名するCSRのパスを入力してください");
        return;
    }

    let file_path = args[1].clone();

    sign(file_path).unwrap();
}