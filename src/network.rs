use bincode;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::components::{Action, LifeformComponent};

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

/// Destination
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Dest {
    Room(String),
    Ip(SocketAddr),
    AllExcept(SocketAddr),
    All,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pack {
    pub cmd: Cmd,
    pub dest: Dest,
}

impl Pack {
    pub fn new(cmd: Cmd, dest: Dest) -> Self {
        Self { cmd, dest }
    }
    pub fn ip(&self) -> Option<SocketAddr> {
        match self.dest {
            Dest::Ip(ip) => return Some(ip),
            _ => return None, // This should never get reached it's panic city
        }
    }

    pub fn from_bin(bin: Vec<u8>) -> Self {
        bincode::deserialize(&bin[..]).unwrap()
    }
    pub fn to_bin(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}
