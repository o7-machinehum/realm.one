use amethyst::{
    ecs::{Component, VecStorage},
    network::*,
    shrev::ReaderId,
};
use serde::{Serialize, Deserialize};
use bincode;

use crate::components::{PlayerInfo, PlayerAction};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Cmd {
    Nothing,
    Connect(String),
    TransferMap(String, String),
    RecivedMap,
    InsertPlayer(PlayerInfo),
    Action(PlayerAction),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pack {
    pub cmd: Cmd,
    id: u32,
    // ip field needed
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
