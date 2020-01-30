use amethyst::{
    derive::SystemDesc,
    ecs::{Write, Read, System, SystemData},
};

use log::info;
use crate::{
    network::{Pack, Cmd},
    map::{Room},
    components::{PlayerComponent, Orientation},
    resources::{PlayerList, IO, MapList},
};

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
    // |
    // |
    // Verify player here
    Some(v[2].to_string())
}

fn insert_map(room: &String, maps: &Vec<Room>) -> String {
    // Find the map in the map vector
    for map in maps {
        if map.name == *room {
            return map.raw.clone();
        }
    }
    "fuck".to_string()
}

fn ready_player_one(ip: Option<SocketAddr>, name: String) -> PlayerComponent {
    info!("Inserting player 1 ({})", name);
   
    // Dig through database to find the correct player by name = name 
    PlayerComponent {
        name,
        ip: ip.unwrap(),
        room: "resources/maps/town.tmx".to_string(),
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
        Read <'a, MapList>,
    );

    fn run(&mut self, (mut io, mut pl, maps): Self::SystemData) {
        for element in io.i.pop() {
            match &element.cmd {
                Cmd::Connect(packet) => {
                    match authenticate(packet.to_string()) {
                        Some(s) => {
                            let player = ready_player_one(element.ip(), s);
                            let ip = player.ip;
                            let map = insert_map(&player.room, &maps.list); 

                            io.o.push(Pack::new(Cmd::TransferMap(player.room.clone(), map), 0, Some(ip))); 
                            io.o.push(Pack::new(Cmd::InsertPlayer(player.clone()), 0, None));
                            
                            // Push the rest of the players
                            for p in &pl.list {
                                io.o.push(Pack::new(Cmd::InsertPlayer(p.clone()), 0, Some(ip)));
                            }
                            
                            pl.list.push(player); 
                        },
                        None => (),
                    }
                },
                
                _ => (io.i.push(element)), 
            }
        }
    }
}
