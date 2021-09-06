use json;

pub fn vec_to_json(data: Vec<u8>) -> Result<json::JsonValue, json::JsonError> {
    let tmp = std::str::from_utf8(&data).unwrap();
    json::parse(tmp)
}

pub fn json_to_string(data: json::JsonValue) -> String {
    data.dump()
}