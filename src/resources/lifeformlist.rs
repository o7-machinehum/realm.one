use amethyst::{
    core::{Transform},
};

use crate::components::{LifeformComponent, LifeformType};
use std::collections::HashMap;
use std::net::SocketAddr;

// Couple of biz guys, remember this
pub struct LifeformList {
    pub list: Vec<Option<LifeformComponent>>,
    ips: HashMap<SocketAddr, usize>,
    ids: HashMap<u64, usize>,
    index: usize,
}

impl Default for LifeformList {
    fn default() -> Self {
        LifeformList::new() 
    }
}

impl LifeformList {
    pub fn new() -> Self {
        Self {
            list: Vec::<Option<LifeformComponent>>::new(),
            ips: HashMap::<SocketAddr, usize>::new(),
            ids: HashMap::<u64, usize>::new(),
            index: 0 as usize, 
        }
    }

    pub fn add(&mut self, player: LifeformComponent) {
        self.ips.insert(player.ip.clone(), self.index); 
        self.ids.insert(player.id(), self.index); 
        self.list.push(Some(player));
        self.index += 1;
    }

    pub fn get_from_ip(&mut self, ip: SocketAddr) -> Option<LifeformComponent> {
        self.list[*self.ips.get(&ip).unwrap()].clone()
    }
    
    /// Get all the IPs in a certain room
    pub fn ip_in_room(&mut self, room: &String) -> Vec<SocketAddr> {
        let mut ip = Vec::<SocketAddr>::new();

        for lifeform in &self.list {
            match lifeform {
                Some(lf) => {
                    if lf.room == *room && lf.kind == LifeformType::Player {
                        ip.push(lf.ip);
                    }
                },
                None => {}
            }
        }
        ip
    }
    
    pub fn get_from_id(&mut self, id: u64) -> Option<LifeformComponent> {
        self.list[*self.ids.get(&id).unwrap()].clone()
    }

    pub fn remove_with_ip(&mut self, ip: SocketAddr) {
        self.list[*self.ips.get(&ip).unwrap()] = None; 
    }

    pub fn remove_with_id(&mut self, id: u64) {
        self.list[*self.ids.get(&id).unwrap()] = None; 
    }

    pub fn replace(&mut self, player: LifeformComponent) {
        let id = player.id(); 
        self.list[*self.ids.get(&id).unwrap()] = Some(player); 
    }

    pub fn get_from_transform(&self, tr: Transform) -> Option<LifeformComponent> {
        for player in self.list.iter() {
            match player {
                Some(pl) => {
                    if pl.trans().translation() == tr.translation() {
                        return Some(pl.clone());
                    } 
                },
                None => (),
            }
        }
        None
    }
}

impl Iterator for LifeformList {
    type Item = LifeformComponent;
    
    fn next(&mut self) -> Option<Self::Item> {
        let i = 0;

        while i < self.index {
            if self.list[i].is_some() {
                return self.list[i].clone();
            }
        }
        None
    }
}
