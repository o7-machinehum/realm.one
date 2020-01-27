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
use crate::components::{PlayerComponent, Orientation, PlayerList};
use std::net::{SocketAddr};

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct AuthSystem;

fn insert_map(ip: Option<SocketAddr>, proof: String) -> Pack {
    info!("Player Connected proof: {}, sending map!", proof);
    let fname = "resources/maps/townCompress2.tmx";
    let mut file = File::open(&fname.to_string()).expect("Unable to open map file"); 
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to convert to string");
    Pack::new(Cmd::TransferMap(fname.to_string(), contents.to_string()), 0, ip)
}

fn ready_player_one(ip: Option<SocketAddr>, player_list: &mut Vec<PlayerComponent>) -> Pack {
    info!("Inserting player 1");
   
    // This should be loaded in the future
    let player1_info = PlayerComponent {
        id: 0,
        ip: ip.unwrap(),
        modified: true,
        name: "Turnip".to_string(),
        room: "Room1".to_string(),
        x: 8.0,
        y: 8.0,
        north: 318,
        east: 306,
        south: 282,
        west: 294,
        orientation: Orientation::North,
    };

    player_list.push(player1_info.clone()); // Add this players to the playerlist
    Pack::new(Cmd::InsertPlayer(player1_info), 0, ip)
}

impl<'a> System<'a> for AuthSystem {
    type SystemData = (
        Write <'a, IO>,
        Write <'a, PlayerList>,
    );

    fn run(&mut self, (mut io, mut pl): Self::SystemData) {
        for element in io.i.pop() {
            match &element.cmd {
                Cmd::Connect(packet) => {
                    io.o.push(insert_map(element.ip(), packet.to_string())); 
                    io.o.push(ready_player_one(element.ip(), &mut pl.list));
                    
                    // Add the other players to the game
                    for player in &pl.list {
                        io.o.push(Pack::new(Cmd::InsertPlayer(player.clone()), 0, element.ip()));
                    }

                    // Then probably add the monsters...
                    // for monster in ml.list {
                    //     io.o.push(Pack::new(Cmd::InsertMonster(monster), 0, ip));
                    // } 
                },
                _ => (io.i.push(element)), 
            }
        }
    }
}
