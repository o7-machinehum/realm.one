use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Component, Entities, Join, System, SystemData, VecStorage, World, WriteStorage},
    network::*,
    shrev::ReaderId,
};
use log::info;

pub struct SpamReader(ReaderId<NetEvent<String>>);

impl Component for SpamReader {
    type Storage = VecStorage<Self>;
}

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct ServerSystem;

impl<'a> System<'a> for ServerSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<String>>,
        WriteStorage<'a, SpamReader>,
        Entities<'a>,
    );

    fn run(&mut self, (mut connections, mut readers, entities): Self::SystemData) {
        let mut count = 0;
        let mut connection_count = 0;

        for (e, connection) in (&entities, &mut connections).join() {
            let reader = readers
                .entry(e)
                .expect("Cannot get reader")
                .or_insert_with(|| SpamReader(connection.register_reader()));

            let mut client_disconnected = false;

            for ev in connection.received_events(&mut reader.0) {
                count += 1;
                match ev {
                    NetEvent::Packet(packet) => info!("{}", packet.content()),
                    NetEvent::Connected(addr) => info!("New Client Connection: {}", addr),
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
        println!(
            "Received {} messages this frame connections: {}",
            count, connection_count
        );
    }
}
