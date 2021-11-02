use ca::save_ca_key;

fn main() {
    match save_ca_key() {
        Ok(_) => {
            println!("キーの作成に成功しました");
        }
        Err(_) => {
            println!("キーの作成に失敗しました");
        }
    }
}