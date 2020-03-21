use serde::{Serialize, Deserialize};
use bincode;
use std::net::{SocketAddr};

use crate::components::{LifeformComponent, Action};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Cmd {
    Ping,
    Connect(String),
    TransferMap(String),
    InsertPlayer(LifeformComponent), 
    Action(Action),
    UpdatePlayer(LifeformComponent),
    RemovePlayer(u64),
}

/// Networking package. addr: None if a broadcast, Some(ip) if direct.
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

    pub fn from_bin(bin: Vec<u8>) -> Self {
        bincode::deserialize(&bin[..]).unwrap() 
    }
     
    pub fn to_bin(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}
