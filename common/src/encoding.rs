use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use anyhow::{Result, anyhow};

pub fn vec_to_struct<T: DeserializeOwned>(data: Vec<u8>) -> Result<T> {
    let tmp = std::str::from_utf8(&data).unwrap();
    match serde_json::from_str(tmp) {
        Ok(data) => Ok(data),
        _ => Err(anyhow!("jsonへのパースに失敗しました"))
    }
}

pub fn struct_to_vec<T: Serialize>(data: T) -> Result<Vec<u8>> {
    match serde_json::to_string(&data) {
        Ok(data) => Ok(data.into_bytes()),
        Err(_) => Err(anyhow!("to string failed"))
    }
}

pub fn value_to_struct<T: DeserializeOwned>(data: serde_json::Value) -> Result<T> {
    match serde_json::from_value(data) {
        Ok(data) => Ok(data),
        Err(_) => Err(anyhow!("jsonを指定されたstructにパースできませんでした"))
    }
}

pub fn struct_to_value<T: Serialize>(data: T) -> Result<serde_json::Value> {
    let value = serde_json::json!(data);
    Ok(value)
}