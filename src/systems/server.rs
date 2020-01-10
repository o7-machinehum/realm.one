use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, System, SystemData, World, WriteStorage},
    network::*,
};
use log::info;
use crate::network;
use crate::network::Pack;

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct ServerSystem;


fn welcome() -> Pack {
    info!("Player Connected, sending map!");
    Pack::send_tmx("map1".to_string(), "Map contents".to_string())
}

fn handle(str: String) -> Pack {
    let pk = network::Pack::from_string(str);
    match pk.cmd {
        network::Cmd::Nothing       => {},
        network::Cmd::TransferMap   => {}, 
        network::Cmd::Connect       => return welcome(),
        network::Cmd::CreateMonster => {},
    }
    network::Pack::nothing() 
}

impl<'a> System<'a> for ServerSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<String>>,
        WriteStorage<'a, network::Reader>,
        Entities<'a>,
    );

    fn run(&mut self, (mut connections, mut readers, entities): Self::SystemData) {
        let mut count = 0;
        let mut connection_count = 0;

        for (e, connection) in (&entities, &mut connections).join() {
            let reader = readers
                .entry(e)
                .expect("Cannot get reader")
                .or_insert_with(|| network::Reader(connection.register_reader()));

            let mut client_disconnected = false;

            let mut str = String::new();
            for ev in connection.received_events(&mut reader.0) {
                count += 1;
                match ev {
                    NetEvent::Packet(packet) => str.push_str(&packet.content().to_string()),
                    NetEvent::Connected(addr) => info!("Client Connected!"), 
                    NetEvent::Disconnected(_addr) => {
                        client_disconnected = true;
                    }
                    _ => {}
                }
            }
            
            if !str.is_empty() {
                let mut pkout = handle(str);
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

            connection_count += 1;
        }
        // println!(
        //     "Received {} messages this frame connections: {}",
        //     count, connection_count
        // );
    }
}
