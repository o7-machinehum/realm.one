use amethyst::{
    ecs::{Component, VecStorage},
    network::*,
    shrev::ReaderId,
};
use serde::{Serialize, Deserialize};
use serde_json;

// eg:
// let mut k = network::pack::new(10);
// let p = k.to_string();
// info!("{}", p);
// let t = network::pack::from_string(p);
// info!("{:?}", t);

pub struct Reader(pub ReaderId<NetEvent<String>>);

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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pack {
    pub cmd: Cmd,
    ints: Vec<u32>,
    floats: Vec<f32>,
    strings: Vec<String>,
}

impl Pack {
    fn fill() -> (Vec<u32>, Vec<f32>, Vec<String>) {
        (Vec::<u32>::new(), Vec::<f32>::new(), Vec::<String>::new())
    }

    pub fn pack_monster(mon_id: u32, xpos: f32, ypos: f32, hp: f32, tile: u32, name: String) -> Self {
        let (mut ints, mut floats, mut strings) =  Pack::fill();
        ints.push(tile);
        floats.push(xpos);
        floats.push(ypos);
        floats.push(hp);
        strings.push(name);
        ints.push(mon_id);
        
        Self {
            cmd: Cmd::CreateMonster,
            ints,
            floats,
            strings,
        }
    }

    pub fn send_tmx(map_name: String, tmx: String) -> Self {
        let (ints, floats, mut strings) =  Pack::fill();
        strings.push(map_name);
        strings.push(tmx);

        Self {
            cmd: Cmd::TransferMap,
            ints,
            floats,
            strings,
        }
    }

    pub fn connect(proof: String) -> Self {
        let (ints, floats, mut strings) =  Pack::fill();
        strings.push(proof);

        Self {
            cmd: Cmd::Connect,
            ints,
            floats,
            strings,
        }
    }
    
    pub fn nothing() -> Self {
        let (ints, floats, strings) =  Pack::fill();

        Self {
            cmd: Cmd::Connect,
            ints, floats, strings,
        }
    }

    pub fn from_string(str: String) -> Self {
       serde_json::from_str(&str).unwrap()
    }
     
    pub fn to_string(&mut self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

// id: 0 - do nothing
