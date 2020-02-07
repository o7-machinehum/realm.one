use crate::components::PlayerComponent;
use std::net::SocketAddr;

// Couple of biz guys, remember this
pub struct PlayerList {
    pub list: Vec<Option<PlayerComponent>>,
    pub ips: HashMap<String, usize>,
    pub name: HashMap<String, usize>,
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
