use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, Write, System, SystemData, World, WriteStorage},
    network::*,
};
use crate::network;
use log::info;
use crate::network::{Pack, Cmd, IO};
use crate::components::PlayerList;

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct ServerSystem; 

impl<'a> System<'a> for ServerSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<Vec<u8>>>,
        WriteStorage<'a, NetIdentity>,
        WriteStorage<'a, network::Reader>,
        Write <'a, IO>,
        Entities<'a>,
    );

    fn run(&mut self, (mut connections, mut ids, mut readers, mut io, entities): Self::SystemData) {
        for (e, connection) in (&entities, &mut connections).join() {
            let reader = readers
                .entry(e)
                .expect("Cannot get reader")
                .or_insert_with(|| network::Reader(connection.register_reader()));
            
            let mut pk_out = Vec::<Pack>::new();

            // Command / Responce below  
            let mut client_disconnected = false;
            for ev in connection.received_events(&mut reader.0) {
                // Get Pack 
                let pack = match ev {
                    NetEvent::Packet(packet) => Some(packet),
                    NetEvent::Connected(_addr) => None,
                    NetEvent::Disconnected(_addr) => {
                        client_disconnected = true;
                        None 
                    },
                    _ => None
                };
                           
                match pack {
                    Some(pack) => io.I.push(Pack::from_bin(pack.content().to_vec())), // Add the pack to the IO vector
                    None => (),
                }
            }
            
            if client_disconnected {
                info!("Client Disconnects");
                entities
                    .delete(e)
                    .expect("Cannot delete connection from world!");
            }
            
            // TODO: There's this member that can be used for vectors. Should use that.
            for mut resp in io.O.pop() {
                info!("Sending a thing"); 
                connection.queue(NetEvent::Packet(NetPacket::reliable_ordered(resp.to_bin(), None)));
            }
        }
    }
}
