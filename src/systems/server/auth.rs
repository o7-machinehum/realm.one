use amethyst::{
    derive::SystemDesc,
    ecs::{Write, Read, System, SystemData},
};

use log::info;
use crate::{
    network::{Pack, Cmd, Dest},
    components::{LifeformComponent},
    resources::{LifeformList, NetInputs, NetOutputs, MapList, LifeformUID},
};

use std::net::{SocketAddr};
use std::iter::{Iterator};

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

fn ready_player_one(ip: Option<SocketAddr>, name: String, id: u64) -> LifeformComponent {
    info!("Inserting player 1 ({})", name);
   
    // Dig through database to find the correct player by name = name 
    LifeformComponent::new_player(name, ip.unwrap(), id)
}

impl<'a> System<'a> for AuthSystem {
    type SystemData = (
        Write <'a, inputs>,
        Write <'a, outputs>,
        Write <'a, LifeformList>,
        Read <'a, MapList>,
        Write <'a, LifeformUID>,
    );

    fn run(&mut self, (mut inputs, mut outputs, mut pl, _maps, mut id): Self::SystemData) {
        for con in inputs.get(Cmdb::Connect) {
            match con.cmd {
                Cmd::Connect // Save this one for another day


            }
            match authenticate(con.to_string()) {
                Some(s) => {
                    let player = ready_player_one(element.ip(), s, id.add());

                    io.o.push(Pack::new(Cmd::TransferMap(player.room.clone()), Dest::Ip(player.ip()))); 
                    io.o.push(Pack::new(Cmd::InsertPlayer1(player.clone()), Dest::Ip(player.ip())));
                    
                    // Push the rest of the players
                    for p in pl.list.iter() {
                        info!("{:?}", p);
                        match p {
                            Some(p) => io.o.push(Pack::new(Cmd::InsertPlayer(p.clone()), Dest::Ip(player.ip()))),
                            None => (),
                        }
                    }
                        
                    info!("{:?}", io.o);
                    
                    pl.add(player); 
                },
                None => (),
            }
        }
    }
}
