use common::db::models::actor::Actor;
use common::enums::ServerType;


#[derive(Clone)]
pub struct State {
    actor: Option<Actor>,
    secret_key: Option<Vec<u8>>,
    opponent_type: Option<ServerType>,
}

impl State {
    /// Create new State
    /// 
    /// stream is tcp stream
    /// 
    /// # Panic
    /// 
    pub fn new(
        actor: Option<Actor>,
        secret_key: Option<Vec<u8>>,
        opponent_type: Option<ServerType>,
    ) -> State {
        State {
            actor,
            secret_key,
            opponent_type,
        }
    }

    pub fn actor(&self) -> Option<Actor> {
        self.actor.clone()
    }

    pub fn secret_key(&self) -> Option<Vec<u8>> {
        self.secret_key.clone()
    }

    pub fn opponent_type(&self) -> Option<ServerType> {
        self.opponent_type.clone()
    }
}