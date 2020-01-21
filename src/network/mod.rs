use amethyst::{
    ecs::{Component, VecStorage},
    network::*,
    shrev::ReaderId,
};
use serde::{Serialize, Deserialize};
use bincode;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::components::{PlayerInfo, Action};

pub struct Reader(pub ReaderId<NetEvent<Vec<u8>>>);

impl Component for Reader {
    type Storage = VecStorage<Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Cmd {
    Nothing,
    Connect(String),
    TransferMap(String, String),
    RecivedMap,
    InsertPlayer(PlayerInfo), 
    Action(Action),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pack {
    pub cmd: Cmd,
    id: u32,
    addr: Option<SocketAddr>,
}

impl Pack {
    pub fn new(cmd: Cmd, id: u32, ip: Option<SocketAddr>) -> Self {
        let ipNew = match ip {
            Some(ip) => ip,
            None => SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3456),
        };

        Self {
            cmd,
            id,
            addr: Some(ipNew), 
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
    pub I: Vec<Pack>,
    pub O: Vec<Pack>,
}

impl IO {
    pub fn new() -> Self {
        Self {
            I: Vec::<Pack>::new(),
            O: Vec::<Pack>::new(),
        }
    }
}

impl Default for IO {
    fn default() -> Self {
        Self {
            I: Vec::<Pack>::new(),
            O: Vec::<Pack>::new(),
        }
    }
}
// id: 0 - do nothing
