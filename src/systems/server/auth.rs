use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Write, System, SystemData, World},
};

use std::{
    fs::File,
};
use std::io::Read;

use log::info;
use crate::network::{Pack, Cmd, IO};
use crate::components::{PlayerInfo};
use std::net::{SocketAddr};

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct AuthSystem;

// fn generate_id(proof: String) -> Pack {
// 
// }

fn insert_map(proof: String, ip: Option<SocketAddr>) -> Pack {
    info!("Player Connected proof: {}, sending map!", proof);
    let fname = "resources/maps/townCompress2.tmx";
    let mut file = File::open(&fname.to_string()).expect("Unable to open map file"); 
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to convert to string");
    Pack::new(Cmd::TransferMap(fname.to_string(), contents.to_string()), 0, ip)
}

fn ready_player_one(ip: Option<SocketAddr>) -> Pack {
    info!("Inserting player 1");
   
    // This should be loaded in the future
    let player1_info = PlayerInfo {
        id: 0,
        modified: true,
        name: "Turnip".to_string(),
        room: "Room1".to_string(),
        x: 8.0,
        y: 8.0,
        no: 318,
        ea: 306,
        so: 282,
        we: 294,
    };

    Pack::new(Cmd::InsertPlayer(player1_info), 0, ip)
}

impl<'a> System<'a> for AuthSystem {
    type SystemData = (
        Write <'a, IO>,
    );

    fn run(&mut self, mut io: Self::SystemData) {
        for element in io.0.i.pop() {
            match &element.cmd {
                Cmd::Connect(packet) => {
                    io.0.o.push(insert_map(packet.to_string(), element.ip())); 
                    // io.0.o.push(ready_player_one(element.ip()));
                    // This also needs to insert ALL the other players
                    // Not just player1
                    // For player in playerlist
                    // Insert players
                    // And monsters?
                },
                _ => (io.0.i.push(element)), 
            }
        }
    }
}
