pub mod utils;

use anyhow::{Result, anyhow};
use strum::{EnumIter, IntoEnumIterator};

use utils::is_valid_actor_type_str;

#[derive(Debug, EnumIter)]
pub enum Actor {
    Entity,
    Role,
    User
}

impl std::fmt::Display for Actor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Actor::Entity => write!(f, "entity"),
            Actor::Role => write!(f, "role"),
            Actor::User => write!(f, "user"),
        }
    }
}

impl Actor {
    pub fn from_string(name: &String) -> Result<Actor> {
        if !is_valid_actor_type_str(name) {
            return Err(anyhow!("名前の形式が正しくありません"));
        }
        
        let actor = Actor::iter().find(|x| &x.to_string() == name);

        match actor {
            Some(actor) => Ok(actor),
            _ => return Err(anyhow!("Actorが見つかりませんでした"))
        }
    }
}