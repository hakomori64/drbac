pub mod utils;

use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;


#[derive(Eq, Debug, FromPrimitive, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActorType {
    Entity = 0,
    Role,
    User
}

impl std::fmt::Display for ActorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorType::Entity => write!(f, "entity"),
            ActorType::Role => write!(f, "role"),
            ActorType::User => write!(f, "user"),
        }
    }
}

impl ActorType {
    pub fn from_i32(x: i32) -> Result<Self> {
        match FromPrimitive::from_i32(x) {
            Some(val) => Ok(val),
            None => return Err(anyhow!(format!("数値{}からActorTypeに変換できませんでした", x)))
        }
    }
}