use common::db::models::actor::Actor;

#[derive(Clone)]
pub struct State {
    pub actor: Option<Actor>,
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
        public_key: Option<Vec<u8>>,
    ) -> State {
        State {
            actor,
            public_key,
        }
    }

    pub fn actor(&self) -> Option<Actor> {
        self.actor.clone()
    }

    pub fn public_key(&self) -> Option<Vec<u8>> {
        self.public_key.clone()
    }
}