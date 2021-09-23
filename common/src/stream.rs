use std::net::{TcpStream, Shutdown};
use std::io::prelude::*;
use anyhow::{Result, anyhow};

pub fn read_stream(stream: &mut TcpStream) -> Result<(Vec<u8>, usize)> {
    match seek_stream(stream) {
        Ok((_, 0)) => {
            Err(anyhow!("コネクションが閉じられました"))
        }
        Ok((data, length)) => {
            Ok((data, length))
        }
        Err(_) => {
            close_stream(stream)?;
            Err(anyhow!("コネクションが閉じられました"))
        }
    }
}

fn seek_stream(stream: &mut TcpStream) -> Result<(Vec<u8>, usize)> {
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
                return Err(anyhow!("データを受け取る過程でエラーが発生しました。"));
            }
        }
    }

    Ok((request_buffer, request_len))
}

pub fn write_stream(stream: &mut TcpStream, data: &[u8]) -> Result<()> {
    if let Err(_) = stream.write(data) {
        return Err(anyhow!("データの送信ができませんでした"));
    }
    
    if let Err(_) = stream.flush() {
        return Err(anyhow!("メッセージの返信中にエラーが発生しました"));
    }

    Ok(())
}

pub fn close_stream(stream: &mut TcpStream) -> Result<()> {
    match stream.shutdown(Shutdown::Both) {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow!("コネクションのクローズに失敗しました"))
    }
}