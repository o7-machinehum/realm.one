pub mod server;
pub mod client;

use amethyst::{
    ecs::{Component, VecStorage},
    network::*,
    shrev::ReaderId,
};
use serde::{Serialize, Deserialize};
use bincode;

use crate::components::PlayerInfo;

// eg:
// let mut k = network::pack::new(10);
// let p = k.to_string();
// info!("{}", p);
// let t = network::pack::from_string(p);
// info!("{:?}", t);

pub struct Reader(pub ReaderId<NetEvent<Vec<u8>>>);

impl Component for Reader {
    type Storage = VecStorage<Self>;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Cmd {
    Nothing,
    Connect(String),
    TransferMap(String, String),
    CreatePlayer(PlayerInfo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pack {
    pub cmd: Cmd,
    id: u32,
}

impl Pack {
    pub fn new(cmd: Cmd, id: u32) -> Self {
        Self {
            cmd,
            id, 
        }
    }

    pub fn from_bin(bin: Vec<u8>) -> Self {
        bincode::deserialize(&bin[..]).unwrap() 
    }
     
    pub fn to_bin(&mut self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

// id: 0 - do nothing
