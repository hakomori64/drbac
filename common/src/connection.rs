use std::net::TcpStream;
use std::str;
use std::sync::Mutex;
use std::convert::TryInto;
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::stream::{read_stream, write_stream, close_stream};
use super::encoding::{vec_to_struct, struct_to_vec};
use super::crypto::traits::CommonKeyCrypto;
use super::messages::common_crypto::CommonKeyCryptoMessage;
use anyhow::{Result, anyhow};

pub struct Connection {
    pub stream: Mutex<Option<TcpStream>>,
    crypto_module: Mutex<Option<Box<dyn CommonKeyCrypto>>>
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            stream: Mutex::new(None),
            crypto_module: Mutex::new(None)
        }
    }

    pub fn set_stream(&mut self, stream: TcpStream) -> Result<()> {
        self.stream = Mutex::new(Some(stream));
        Ok(())
    }

    pub fn set_crypto_module(&mut self, module: Box<dyn CommonKeyCrypto>) -> Result<()> {
        self.crypto_module = Mutex::new(Some(module));
        Ok(())
    }

    pub fn read(&self) -> Result<Vec<u8>> {

        if self.stream.lock().unwrap().is_none() {
            return Err(anyhow!("streamが設定されていません"));
        }

        let stream: &mut TcpStream = &mut self.stream.lock().unwrap().as_ref().unwrap().try_clone().unwrap();
        let data = match read_stream(stream) {
            Ok(data) => data.0,
            Err(_) => return Err(anyhow!("読み込みに失敗しました"))
        };

        println!("receiving data {}...", str::from_utf8(&data[..]).unwrap());

        if self.crypto_module.lock().unwrap().is_none() {
            return Ok(data);
        }


        let data = match vec_to_struct::<CommonKeyCryptoMessage>(data) {
            Ok(data) => data,
            Err(_) => return Err(anyhow!("jsonとしてのパースに失敗しました"))
        };
        
        self.crypto_module.lock().unwrap().as_ref().unwrap().decipher_message(&data.cipher_text[..], &data.nonce[..])
    }

    pub fn read_json<T: DeserializeOwned>(&self) -> Result<T> {
        let data = match self.read() {
            Ok(data) => data,
            Err(_) => return Err(anyhow!("error occured"))
        };
        vec_to_struct(data)
    }

    pub fn write(&self, data: &[u8]) -> Result<()> {
        println!("sending data {}...", str::from_utf8(data).unwrap());
        if self.stream.lock().unwrap().is_none() {
            return Err(anyhow!("streamの設定に失敗しました"));
        }

        //let stream: &mut TcpStream = &mut self.stream.unwrap().try_clone().unwrap();
        if self.crypto_module.lock().unwrap().is_none() {
            return write_stream(&mut self.stream.lock().unwrap().as_ref().unwrap().try_clone().unwrap(), data);
        }

        let (cipher_text, nonce) = match self.crypto_module.lock().unwrap().as_ref().unwrap().cipher_message(data) {
            Ok(data) => data,
            Err(_) => return Err(anyhow!("暗号化に失敗しました"))
        };

        let data = CommonKeyCryptoMessage::new(
            cipher_text,
            nonce.try_into().unwrap(),
        );

        write_stream(&mut self.stream.lock().unwrap().as_ref().unwrap().try_clone().unwrap(), &struct_to_vec(data).unwrap()[..])
    }

    pub fn write_json<T: Serialize>(&self, data: T) -> Result<()> {
        self.write(&struct_to_vec(data).unwrap()[..])
    }

    pub fn close(&self, message: &str) -> Result<()> {
        if self.stream.lock().unwrap().is_none() {
            return Ok(());
        }
        let stream: &mut TcpStream = &mut self.stream.lock().unwrap().as_ref().unwrap().try_clone().unwrap();
        close_stream(stream, message);
        Ok(())
    }
}