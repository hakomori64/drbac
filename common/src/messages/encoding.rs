use serde_json::Value;
use serde::{Serialize, Deserialize, Serializer, Deserializer, de::Visitor, de::MapAccess};
use serde::de::DeserializeOwned;
use crate::encoding::{
    value_to_struct
};

pub fn value_to_data<'de, T: DeserializeOwned, M: MapAccess<'de>>(value: Value) -> Result<T, M::Error> {
    match value_to_struct::<T>(value) {
        Ok(data) => Ok(data),
        Err(_) => return Err(serde::de::Error::custom("ヘッダとデータの構造が一致しませんでした"))
    }
}