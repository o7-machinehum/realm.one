use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, System, SystemData, World, Write, WriteStorage},
    network::*,
};
use log::info;
use crate::network;
use crate::network::Pack;
use crate::resources::ClientStatus;

/// A simple system that sends a ton of messages to all connections.
/// In this case, only the server is connected.
#[derive(SystemDesc)]
pub struct ClientSystem;

impl<'a> System<'a> for ClientSystem {
    type SystemData = (
        Write<'a, ClientStatus>, 
        WriteStorage<'a, NetConnection<Vec::<u8>>>,
        WriteStorage<'a, network::Reader>,
        Entities<'a>,
    );

    fn run(&mut self, (mut status, mut connections, mut readers, entities): Self::SystemData) {
        for (e, connection) in (&entities, &mut connections).join() {
            if !status.connected {
                 info!("Authenticating");
                 let mut packet = Pack::connect("pubkey or some shit".to_string());  
                 connection.queue(NetEvent::Packet(NetPacket::unreliable(packet.to_bin())));
                 status.connected = true;
            }
            
            else {
                let reader = readers
                    .entry(e)
                    .expect("Cannot get reader")
                    .or_insert_with(|| network::Reader(connection.register_reader()));

                let mut recv = Vec::<u8>::new();
                
                for ev in connection.received_events(&mut reader.0) {
                    match ev {
                        NetEvent::Packet(packet) => {}, //recv.append(packet.content_mut()),
                        NetEvent::Connected(addr) => info!("Client Connected! {}", addr), 
                        NetEvent::Disconnected(_addr) => {}
                        _ => {}
                    }
                    // info!("{:?}", Pack::from_bin(recv));
                }
            
                // if !str.is_empty() {
                //     let mut pkout = handle(str);
                //     if pkout.cmd != network::Cmd::Nothing{ 
                //         connection.queue(NetEvent::Packet(NetPacket::unreliable(pkout.to_string())));
                //     } 
                // }
            }
        }
    }
}
