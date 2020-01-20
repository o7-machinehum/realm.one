use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, Write, System, SystemData, World, WriteStorage},
    network::*,
};
use crate::network;
use log::info;
use crate::network::{Pack, server, Cmd};
use crate::components::PlayerList;

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct ServerSystem {
    pub new_players: Vec<Pack>, 
}

impl<'a> System<'a> for ServerSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<Vec<u8>>>,
        WriteStorage<'a, network::Reader>,
        Write<'a, PlayerList>,
        Entities<'a>,
    );

    fn run(&mut self, (mut connections, mut readers, mut players, entities): Self::SystemData) {
        // Add new players 
        // for player in &mut players.list {     // For all the players in game
        //     if player.modified {    // If one has been modified
        //         self.new_players.push(Pack::new(Cmd::CreatePlayer(player.clone()), 0)); // Send out the new pack
        //         info!("Inserting Player {:?}", self.new_players); 
        //         player.modified = false; 
        //     }
        // }

        for (e, connection) in (&entities, &mut connections).join() {
            let reader = readers
                .entry(e)
                .expect("Cannot get reader")
                .or_insert_with(|| network::Reader(connection.register_reader()));
            
            let mut pk_out = Vec::<Pack>::new();
            // pk_out.append(&mut self.new_players.clone()); 

            // Command / Responce below  
            let mut client_disconnected = false;
            for ev in connection.received_events(&mut reader.0) {
                // Get Pack 
                let rtn = match ev {
                    NetEvent::Packet(packet) => Some(packet),
                    NetEvent::Connected(_addr) => None,
                    NetEvent::Disconnected(_addr) => {
                        client_disconnected = true;
                        None 
                    },
                    _ => None
                };
                
                // Process Pack
                let mut out = match rtn {
                    Some(rtn) => server::handle(rtn.content().to_vec()),
                    None => None, 
                };

                // Add to vector of responces 
                match out {
                    Some(mut out) => {
                        match out.cmd {
                            Cmd::CreatePlayer(ref mut pl) => pl.append(&mut players.list.clone()), 
                            _ => (), 
                        }
                        pk_out.push(out);
                    },
                    None => {},
                }
            }
            
            if client_disconnected {
                info!("Client Disconnects");
                entities
                    .delete(e)
                    .expect("Cannot delete connection from world!");
            }
            
            // Respond
            // TODO: There's this member that can be used for vectors. Should use that.
            for mut resp in pk_out {
                connection.queue(NetEvent::Packet(NetPacket::reliable_ordered(resp.to_bin(), None)));
            }
        }
        // self.new_players = Vec::<Pack>::new();
    }
}
