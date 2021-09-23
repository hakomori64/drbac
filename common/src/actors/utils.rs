use anyhow::{Result, anyhow};
use regex::Regex;
use std::path::PathBuf;
use super::super::pki;
use strum::IntoEnumIterator;

use super::Actor;

pub fn is_valid_actor_format(name: &String) -> bool {

    let re = Regex::new(r#"^[^/\\. ]+(\.[^/\\. ]+)?$"#).unwrap();
    re.is_match(name.as_str())
}

pub fn is_valid_format(actor_type: &Actor, name: &String) -> bool {
    match actor_type {
        Actor::Entity => {
            Regex::new(r#"^[^/\\. ]+$"#).unwrap()
                 .is_match(name.as_str())
        }
        Actor::Role | Actor::User => {
            Regex::new(r#"^[^/\\. ]+\.[^/\\. ]+$"#).unwrap()
                 .is_match(name.as_str())
        }
    }
}

pub fn craft_base_dir(actor_type: &Actor, name: &String) -> Result<PathBuf> {
    if !(is_valid_actor_format(&name) && is_valid_format(&actor_type, &name)) {
        return Err(anyhow!("渡された名前の形が適当ではありません"));
    }
    match actor_type {
        Actor::Entity => Ok(["actors", name.as_str()].iter().collect()),
        Actor::Role | Actor::User => {
            let name: Vec<&str> = name.as_str().split(".").collect();
            Ok(["actors", name[0], format!("{}s", actor_type.to_string().as_str()).as_str(), name[1]].iter().collect())
        }
    }
}

pub fn get_public_key_path(actor_type: &Actor, name: &String) -> Result<PathBuf> {
    let key_path = &mut craft_base_dir(actor_type, name)?;
    key_path.push(pki::PUBLIC_FILE_NAME);
    Ok(key_path.clone())
}

pub fn get_key_content(key_path: PathBuf) -> Result<Vec<u8>> {
    if !key_path.exists() {
        return Err(anyhow!("キーファイルが存在しません"));
    }

    pki::read_pem(&key_path)
}

pub fn get_secret_key_path(actor_type: &Actor, name: &String) -> Result<PathBuf> {
    let key_path = &mut craft_base_dir(actor_type, name)?;
    key_path.push(pki::SECRET_FILE_NAME);
    Ok(key_path.clone())
}

/// returns (secret key path, public key path)
/// 
/// # panic
/// this function panics when name format does not follow actor_type
pub fn get_key_paths(actor_type: &Actor, name: &String) -> Result<(PathBuf, PathBuf)> {
    Ok((get_secret_key_path(actor_type, name)?, get_public_key_path(actor_type, name)?))
}

/// returns (secret key contents, public key contents)
/// 
/// # panic
/// this function panics when name format does not follow actor_type
pub fn get_key_contents(actor_type: &Actor, name: &String) -> Result<(Vec<u8>, Vec<u8>)> {
    let (secret_key_path, public_key_path) = get_key_paths(actor_type, name)?;
    Ok((get_key_content(secret_key_path)?, get_key_content(public_key_path)?))
}

pub fn is_valid_actor_type_str(actor_type_str: &String) -> bool {
    Actor::iter().map(|value| value.to_string()).collect::<String>().contains(actor_type_str)
}