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

fn authenticate(proof: String) -> Option<String> {
    let v: Vec<&str> = proof.rsplit(' ').collect();

    if v.len() != 3 {
        info!("Proof Package not correct format {:?}", v); 
        return None;
    }
    
    info!("Name: {}, time: {}, Signature: {}", v[2], v[1], v[0]); 
    // Verify player here
    Some(v[2].to_string())
}

fn insert_map(ip: Option<SocketAddr>, room: String) -> Pack {
    let mut file = File::open(&room).expect("Unable to open map file"); 
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to convert to string");
    Pack::new(Cmd::TransferMap(room, contents.to_string()), 0, ip)
}

fn ready_player_one(ip: Option<SocketAddr>, name: String) -> PlayerComponent {
    info!("Inserting player 1 ({})", name);
   
    // Dig through database to find the correct player by name = name 
    PlayerComponent {
        id: 0,
        ip: ip.unwrap(),
        modified: true,
        name,
        room: "resources/maps/townCompress2.tmx".to_string(),
        x: 8.0,
        y: 8.0,
        north: 318,
        east: 306,
        south: 282,
        west: 294,
        orientation: Orientation::North,
    }
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
                    match authenticate(packet.to_string()) {
                        Some(s) => {
                            let player = ready_player_one(element.ip(), s);
                            let ip = player.ip;
                            io.o.push(insert_map(Some(ip), player.room.clone())); 
                            io.o.push(Pack::new(Cmd::InsertPlayer(player), 0, Some(ip)));
                        },
                        None => (),
                    }
                    // Add the other players to the game
                    // for player in &pl.list {
                    //     io.o.push(Pack::new(Cmd::InsertPlayer(player.clone()), 0, element.ip()));
                    // }

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
