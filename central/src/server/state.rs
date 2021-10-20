use common::db::models::actor::Actor;

#[derive(Clone)]
pub struct State {
    pub actor: Option<Actor>,
    pub secret_key: Option<Vec<u8>>,
    pub public_key: Option<Vec<u8>>
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
        public_key: Option<Vec<u8>>,
    ) -> State {
        State {
            actor,
            secret_key,
            public_key,
        }
    }

    pub fn actor(&self) -> Option<Actor> {
        self.actor.clone()
    }

    pub fn secret_key(&self) -> Option<Vec<u8>> {
        self.secret_key.clone()
    }

    pub fn public_key(&self) -> Option<Vec<u8>> {
        self.public_key.clone()
    }
}