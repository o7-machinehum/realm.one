use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, System, SystemData, World, WriteStorage},
    network::*,
};
use log::info;
use crate::network;
use crate::network::Pack;
use crate::network::server;

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct ServerSystem;

impl<'a> System<'a> for ServerSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<Vec<u8>>>,
        WriteStorage<'a, network::Reader>,
        Entities<'a>,
    );

    fn run(&mut self, (mut connections, mut readers, entities): Self::SystemData) {
        for (e, connection) in (&entities, &mut connections).join() {
            let reader = readers
                .entry(e)
                .expect("Cannot get reader")
                .or_insert_with(|| network::Reader(connection.register_reader()));

            let mut client_disconnected = false;
            // let mut recv = Vec::<u8>::new();

            for ev in connection.received_events(&mut reader.0) {
                // Get Pack 
                let rtn = match ev {
                    NetEvent::Packet(packet) => Some(packet),
                    NetEvent::Connected(addr) => None,
                    NetEvent::Disconnected(_addr) => None,
                    _ => None
                };
                
                // Process Pack
                let out = match rtn {
                    Some(rtn) => server::handle(rtn.content().to_vec()),
                    None => None, 
                };

                // Respond to pack (or not)
                match out {
                    Some(mut out) => connection.queue(NetEvent::Packet(NetPacket::reliable_ordered(out.to_bin(), None))),
                    None => {},    
                }

            }
            
            if client_disconnected {
                println!("Client Disconnects");
                entities
                    .delete(e)
                    .expect("Cannot delete connection from world!");
            }
        }
    }
}
