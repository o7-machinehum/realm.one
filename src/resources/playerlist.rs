use crate::components::PlayerComponent;
use std::collections::HashMap;
use std::net::SocketAddr;

// Couple of biz guys, remember this
pub struct PlayerList {
    pub list: Vec<Option<PlayerComponent>>,
    ips: HashMap<SocketAddr, usize>,
    name: HashMap<String, usize>,
    index: usize,
}

impl Default for PlayerList {
    fn default() -> Self {
        PlayerList::new() 
    }

}

impl PlayerList {
    pub fn new() -> Self {
        Self {
            list: Vec::<Option<PlayerComponent>>::new(),
            ips: HashMap::<SocketAddr, usize>::new(),
            name: HashMap::<String, usize>::new(),
            index: 0 as usize, 
        }
    }

    pub fn add(&mut self, player: PlayerComponent) {
        self.ips.insert(player.ip.clone(), self.index); 
        self.name.insert(player.name.clone(), self.index); 
        self.list.push(Some(player));
        self.index += 1;
    }

    pub fn get_from_ip(&mut self, ip: SocketAddr) -> Option<PlayerComponent> {
        self.list[*self.ips.get(&ip).unwrap()].clone()
    }

    pub fn remove_with_ip(&mut self, ip: SocketAddr) {
        self.list[*self.ips.get(&ip).unwrap()] = None; 
    }

    pub fn replace(&mut self, player: PlayerComponent) {
        let name = player.name.clone();
        self.list[*self.name.get(&name).unwrap()] = Some(player); 
    } 
}

impl Iterator for PlayerList {
    type Item = PlayerComponent;
    
    fn next(&mut self) -> Option<Self::Item> {
        let mut i = 0;

        while i < self.index {
            if self.list[i].is_some() {
                return self.list[i].clone();
            }
        }
        None
    }
}
