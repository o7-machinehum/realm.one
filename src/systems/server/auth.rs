use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, Write, System, SystemData, World, WriteStorage},
    network::*,
};

use std::{
    fs::File,
};
use std::io::Read;

use crate::network;
use log::info;
use crate::network::{Pack, Cmd, IO};
use crate::components::{PlayerList, PlayerAction, PlayerInfo, Action};

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct AuthSystem;

fn welcome(proof: String) -> Pack {
    info!("Player Connected proof: {}, sending map!", proof);
    let fname = "resources/maps/townCompress2.tmx";
    let mut file = File::open(&fname.to_string()).expect("Unable to open map file"); 
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to convert to string");
    Pack::new(Cmd::TransferMap(fname.to_string(), contents.to_string()), 0)
}

fn ready_player_one() -> Pack {
    info!("Inserting player 1");
   
    // This should be loaded in the future
    let player1_info = PlayerInfo {
        id: 0,
        modified: true,
        act: PlayerAction::new(0, Action::Nothing),
        name: "Turnip".to_string(),
        room: "Room1".to_string(), 
        x: 8.0,        
        y: 8.0, 
        no: 318,        
        ea: 306, 
        so: 282,
        we: 294, 
    };

    Pack::new(Cmd::InsertPlayer(player1_info), 0)
}

impl<'a> System<'a> for AuthSystem {
    type SystemData = (
        Write <'a, IO>,
    );

    fn run(&mut self, (mut io): Self::SystemData) {
        for element in io.0.I.pop() {
            match &element.cmd {
                Cmd::Connect(packet) => {
                    io.0.O.push(welcome(packet.to_string())); 
                    io.0.O.push(ready_player_one());
                },
                _ => (io.0.I.push(element)), 
            }
        }
    } 
}
