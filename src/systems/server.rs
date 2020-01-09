use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, System, SystemData, World, WriteStorage},
    network::*,
};
use log::info;
use crate::network;

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct ServerSystem;

fn handle(str: String) {
    let pk = network::Pack::from_string(str);
    match pk.cmd {
        network::Cmd::Nothing       => {},
        network::Cmd::Connect       => info!("Player Connected!"),
        network::Cmd::CreateMonster => {},
    }
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

            for ev in connection.received_events(&mut reader.0) {
                count += 1;
                match ev {
                    NetEvent::Packet(packet) => handle(packet.content().to_string()),
                    NetEvent::Connected(addr) => info!("Client Connected!"), //greet(),
                    NetEvent::Disconnected(_addr) => {
                        client_disconnected = true;
                    }
                    _ => {}
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
