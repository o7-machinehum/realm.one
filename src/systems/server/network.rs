use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, Write, System, SystemData, World, WriteStorage},
    network::*,
};
use crate::network;
use crate::network::{Pack, server, Cmd};
use crate::components::PlayerList;

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct ServerSystem;

impl<'a> System<'a> for ServerSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<Vec<u8>>>,
        WriteStorage<'a, network::Reader>,
        Write<'a, PlayerList>,
        Entities<'a>,
    );

    fn run(&mut self, (mut connections, mut readers, mut players, entities): Self::SystemData) {
        // Add new players 
        let mut new_players = Vec::<Pack>::new();
        for player in &mut players.list {     // For all the players in game
            if player.modified {    // If one has been modified
                new_players.push(Pack::new(Cmd::CreatePlayer(player.clone()), 0)); // Send out the new pack
                player.modified = false; 
            }
        }

        for (e, connection) in (&entities, &mut connections).join() {
            let reader = readers
                .entry(e)
                .expect("Cannot get reader")
                .or_insert_with(|| network::Reader(connection.register_reader()));
            
            let mut pk_out = Vec::<Pack>::new();
            pk_out.append(&mut new_players.clone()); 
            // Command / Responce below
            for ev in connection.received_events(&mut reader.0) {
                // Get Pack 
                let rtn = match ev {
                    NetEvent::Packet(packet) => Some(packet),
                    NetEvent::Connected(_addr) => None,
                    NetEvent::Disconnected(_addr) => None,
                    _ => None
                };
                
                // Process Pack
                let out = match rtn {
                    Some(rtn) => server::handle(rtn.content().to_vec()),
                    None => None, 
                };

                // Add to vector of responces 
                match out {
                    Some(out) => pk_out.push(out), 
                    None => {},    
                }
            }
            
            // Respond
            // TODO: There's this member that can be used for vectors. Should use that.
            for mut resp in pk_out {
                connection.queue(NetEvent::Packet(NetPacket::reliable_ordered(resp.to_bin(), None)));
            }
        }
    }
}
