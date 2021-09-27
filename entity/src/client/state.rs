use common::actors::Actor;


#[derive(Clone)]
pub struct State {
    name: Option<String>,
    actor_type: Option<Actor>,
    secret_key: Option<Vec<u8>>
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
            name,
            actor_type,
            secret_key,
        }
    }
}