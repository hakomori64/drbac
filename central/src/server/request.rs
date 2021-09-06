use common::stream::{close_stream};
use common::encoding::{vec_to_json};

use super::state::State;


pub fn handle_request(state: &mut State, data: Vec<u8>) -> Result<(), &'static str> {
    let data = match vec_to_json(data) {
        Ok(data) => { data }
        Err(_) => {
            close_stream(&mut state.stream, "不正な文字列です");
            return Err("不正な文字列が渡された");
        }
    };
    println!("{}", data);
    println!("{}", data["type"]);
    match data["type"].as_str() {
        Some(request_type) => {
            match request_type {
                "CRYPTO_CHANNEL_REQ1" => {}
                "AUTH_IDENTIFICATE_REQ1" => {}
                "WHOAMI_REQ1" => {}
                "DELEGATE_ROLE_REQ1" => {}
                "SEARCH_ROLE_REQ1" => {}
                _ => {
                    close_stream(&mut state.stream, "認識できないリクエストです");
                    return Err("認識できないリクエストが渡された");
                }
            }
        }
        None => {
            close_stream(&mut state.stream, "リクエストのタイプが存在しません");
            return Err("リクエストのタイプが存在しません");
        }
    }
    Ok(())
}