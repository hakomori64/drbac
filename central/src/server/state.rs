use common::actors::Actor;

#[derive(Clone)]
pub struct State {
    pub name: Option<String>,
    pub actor_type: Option<Actor>,
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
        name: Option<String>,
        actor_type: Option<Actor>,
        secret_key: Option<Vec<u8>>,
    ) -> State {
        State {
            name: None,
            actor_type: None,
            public_key: None,
        }
    }
}