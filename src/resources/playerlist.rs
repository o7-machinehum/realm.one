use crate::components::PlayerComponent;

pub struct PlayerList {
    pub list: Vec<PlayerComponent>,
}

impl Default for PlayerList {
    fn default() -> Self {
        Self{ list: Vec::new(), } 
    }
}

