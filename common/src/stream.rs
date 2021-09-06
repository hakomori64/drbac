use std::net::{TcpStream, Shutdown};
use std::io::prelude::*;

pub fn read_stream(stream: &mut TcpStream) -> Result<(Vec<u8>, usize), &str> {
    match seek_stream(stream) {
        Ok((_, 0)) => {
            Err("コネクションが閉じられました")
        }
        Ok((data, length)) => {
            Ok((data, length))
        }
        Err(_) => {
            close_stream(stream, "something went wrong");
            Err("コネクションが閉じられました")
        }
    }
}

fn seek_stream(stream: &mut TcpStream) -> Result<(Vec<u8>, usize), &str> {
    let buffer_size = 512;
    let mut request_buffer = vec![];
    let mut request_len = 0usize;

    loop {
        let mut buffer = vec![0u8; buffer_size];
        match stream.read(&mut buffer) {
            Ok(n) => {
                request_len += n;
                request_buffer.extend_from_slice(&mut buffer[0..n]);
                if n < buffer_size {
                    break;
                }
            }
            _ => {
                return Err("データを受け取る過程でエラーが発生しました。");
            }
        }
    }

    Ok((request_buffer, request_len))
}

pub fn write_stream(stream: &mut TcpStream, data: String) -> Result<(), &str> {
    if let Err(_) = stream.write(data.as_bytes()) {
        return Err("データの送信ができませんでした");
    }
    
    if let Err(_) = stream.flush() {
        return Err("メッセージの返信中にエラーが発生しました");
    }

    Ok(())
}

pub fn close_stream(stream: &mut TcpStream, message: &str) {
    write_stream(stream, message.to_string()).unwrap();
    stream.shutdown(Shutdown::Both).expect("コネクションのクローズに失敗しました");
}