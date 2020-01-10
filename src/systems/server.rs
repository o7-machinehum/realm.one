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
            let mut recv = Vec::<u8>::new();

            for ev in connection.received_events(&mut reader.0) {
                match ev {
                    NetEvent::Packet(packet) =>  recv.append(packet.content_mut()),
                    NetEvent::Connected(addr) => info!("Client Connected!, {}", addr), 
                    NetEvent::Disconnected(_addr) => {
                        client_disconnected = true;
                    }
                    _ => {}
                }
            }
            
            info!("{:?}", reader.0);
            
            if recv.is_empty() {
                let mut pkout = server::handle(recv);
                if pkout.cmd != network::Cmd::Nothing{ 
                    connection.queue(NetEvent::Packet(NetPacket::reliable_ordered(pkout.to_bin(), None)));
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
