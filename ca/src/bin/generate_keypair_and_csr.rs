use std::env;
use ca::generate_keypair_and_csr;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("名前、サーバータイプを入力してください");
        return;
    }

    let name = args[1].clone();
    let server_type = args[2].clone();

    generate_keypair_and_csr(name, server_type).unwrap();
}