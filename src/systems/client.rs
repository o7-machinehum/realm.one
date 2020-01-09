use amethyst::{
    core::{SystemDesc, Time},
    derive::SystemDesc,
    ecs::{Component, Entities, Join, Read, System, SystemData, World, Write, WriteStorage, VecStorage},
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
        WriteStorage<'a, NetConnection<String>>,
        WriteStorage<'a, network::Reader>,
        Entities<'a>,
    );

    fn run(&mut self, (mut status, mut connections, mut readers, entities): Self::SystemData) {
        for (conn) in (&mut connections ).join() {
            if !status.connected {
                info!("Authenticating");
                let mut packet = Pack::connect("pubkey or some shit".to_string());  
                conn.queue(NetEvent::Packet(NetPacket::unreliable(packet.to_string())));
                status.connected = true;
            }
        }
    }
}
