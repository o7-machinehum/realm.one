use amethyst::{
    ecs::{Component, VecStorage},
    shrev::ReaderId,
};
use serde::{Serialize, Deserialize};
use bincode;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::components::{PlayerComponent, Action};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Cmd {
    Ping,
    Connect(String),
    TransferMap(String, String),
    InsertPlayer(PlayerComponent), 
    Action(Action),
    UpdatePlayer(PlayerComponent),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pack {
    pub cmd: Cmd,
    id: u32,
    pub addr: Option<SocketAddr>,
}

impl Pack {
    pub fn new(cmd: Cmd, id: u32, ip: Option<SocketAddr>) -> Self {
        Self {
            cmd,
            id,
            addr: ip, 
        }
    }
    
    pub fn ip(&self) -> Option<SocketAddr> {
        self.addr
    }

    pub fn update_ip(&mut self, ip: SocketAddr) {
        self.addr = Some(ip);
    }

    pub fn from_bin(bin: Vec<u8>) -> Self {
        bincode::deserialize(&bin[..]).unwrap() 
    }
     
    pub fn to_bin(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}


/// IO resource, buff for inputs and outputs
pub struct IO {
    pub i: Vec<Pack>,
    pub o: Vec<Pack>,
}

impl IO {
    pub fn new() -> Self {
        Self {
            i: Vec::<Pack>::new(),
            o: Vec::<Pack>::new(),
        }
    }
}

impl Default for IO {
    fn default() -> Self {
        Self {
            i: Vec::<Pack>::new(),
            o: Vec::<Pack>::new(),
        }
    }
}
// id: 0 - do nothing
