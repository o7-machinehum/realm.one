use serde::{Serialize, Deserialize};
use bincode;
use std::net::{SocketAddr};
use crate::map::Room;

use crate::components::{LifeformComponent, Action};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Cmd {
    Ping,
    Connect(String),
    TransferMap(String),
    InsertPlayer(LifeformComponent), 
    InsertPlayer1(LifeformComponent), 
    Action(Action),
    UpdatePlayer(LifeformComponent),
    RemovePlayer(u64),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Cmdb {
    Ping,
    Connect,
    TransferMap,
    InsertPlayer, 
    InsertPlayer1, 
    Action,
    UpdatePlayer,
    RemovePlayer,
}

/// Destination
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Dest {
    Room(String),
    Ip(SocketAddr),
    All,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pack {
    pub cmd: Cmd,
    pub dest: Dest,
}

impl Pack {
    pub fn new(cmd: Cmd, dest: Dest) -> Self {
        Self {
            cmd,
            dest,
        }
    }
    
    pub fn ip(&self) -> Option<SocketAddr> {
        match self.dest {
            Dest::Ip(ip) => return Some(ip),
            _ => return(None), // This should never get reached it's panic city
        }
    }

    pub fn from_bin(bin: Vec<u8>) -> Self {
        bincode::deserialize(&bin[..]).unwrap() 
    }
     
    pub fn to_bin(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}
