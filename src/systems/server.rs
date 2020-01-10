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
        WriteStorage<'a, NetConnection<String>>,
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

            let mut str = String::new();
            for ev in connection.received_events(&mut reader.0) {
                match ev {
                    NetEvent::Packet(packet) => str.push_str(&packet.content().to_string()),
                    NetEvent::Connected(addr) => info!("Client Connected!, {}", addr), 
                    NetEvent::Disconnected(_addr) => {
                        client_disconnected = true;
                    }
                    _ => {}
                }
            }
            
            if !str.is_empty() {
                let mut pkout = server::handle(str);
                if pkout.cmd != network::Cmd::Nothing{ 
                    connection.queue(NetEvent::Packet(NetPacket::unreliable(pkout.to_string())));
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
