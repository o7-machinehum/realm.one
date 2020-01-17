pub mod server;
pub mod client;

use amethyst::{
    ecs::{Component, VecStorage},
    network::*,
    shrev::ReaderId,
};
use serde::{Serialize, Deserialize};
use bincode;

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

#[derive(PartialEq)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Cmd {
    Nothing,
    Connect,
    TransferMap,
    CreateMonster,  // Create a monsters
    CreatePlayer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pack {
    pub cmd: Cmd,
    id: u32,
    ints: Vec<u32>,
    floats: Vec<f32>,
    strings: Vec<String>,
    ser_struct: Vec<u8>,
}

impl Pack {
    fn fill() -> (Vec<u32>, Vec<f32>, Vec<String>, Vec<u8>) {
        (Vec::<u32>::new(), Vec::<f32>::new(), Vec::<String>::new(), Vec::<u8>::new())
    }

    pub fn send_tmx(map_name: String, tmx: String) -> Self {
        let (ints, floats, mut strings, ser_struct) =  Pack::fill();
        strings.push(map_name);
        strings.push(tmx);

        Self {
            cmd: Cmd::TransferMap,
            id: 0, 
            ints,
            floats,
            strings,
            ser_struct,
        }
    }

    pub fn connect(proof: String) -> Self {
        let (ints, floats, mut strings, ser_struct) =  Pack::fill();
        strings.push(proof);

        Self {
            cmd: Cmd::Connect,
            id: 0, 
            ints,
            floats,
            strings,
            ser_struct,
        }
    }
    
    pub fn nothing() -> Self {
        let (ints, floats, strings, ser_struct) =  Pack::fill();

        Self {
            cmd: Cmd::Connect,
            id: 0, 
            ints, floats, strings,
            ser_struct,
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
