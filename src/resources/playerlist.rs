use crate::components::PlayerComponent;
use std::net::SocketAddr;

pub struct PlayerList {
    pub list: Vec<PlayerComponent>,
}

impl Default for PlayerList {
    fn default() -> Self {
        Self{ list: Vec::new(), } 
    }
}

impl PlayerList {
    pub fn remove(&mut self, ip: SocketAddr) {
        self.list.retain(|x| x.ip != ip); 
    }
}
