use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, System, SystemData, World, Write, WriteStorage},
    network::*,
    shrev::EventChannel
};
use log::info;
use crate::network;
use crate::network::Pack;
use crate::network::client;
use crate::resources::ClientStatus;
use crate::map::Room;
use crate::events::{Events};

/// A simple system that sends a ton of messages to all connections.
/// In this case, only the server is connected.
#[derive(SystemDesc)]
pub struct ClientSystem;

impl<'a> System<'a> for ClientSystem {
    type SystemData = (
        Write<'a, ClientStatus>, 
        Write<'a, EventChannel<Events>>,
        WriteStorage<'a, NetConnection<Vec::<u8>>>,
        WriteStorage<'a, network::Reader>,
        Write<'a, Room>,
        Entities<'a>,
    );

    fn run(&mut self, (mut status, mut events, mut connections, mut readers, mut room, entities): Self::SystemData) {
        for (e, connection) in (&entities, &mut connections).join() {
            let reader = readers
                .entry(e)
                .expect("Cannot get reader")
                .or_insert_with(|| network::Reader(connection.register_reader()));
            
            if !status.connected {
                 info!("Authenticating");
                 let mut packet = Pack::connect("pubkey or some shit".to_string());  
                 connection.queue(NetEvent::Packet(NetPacket::unreliable(packet.to_bin())));
                 status.connected = true;
            }
            
            else {
                let mut recv = Vec::<Pack>::new();
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
                        Some(rtn) => client::handle(rtn.content().to_vec(), &entities),
                        None => (None, None), 
                    };

                    // Add to vector of udp responces 
                    match out.0 {
                        Some(mut out) => recv.push(out),
                        None => {},
                    }
                    
                    // Then write the event to the event channel
                    match out.1 {
                        Some(out) => {
                            match out {
                                Events::NewMap(map) => room.change(map), 
                            }
                        // events.single_write(out);
                        },
                        None => {},
                    }
                }
                
                // Respond
                // TODO: There's this member that can be used for vectors. Should use that.
                for mut resp in recv {
                    connection.queue(NetEvent::Packet(NetPacket::reliable_ordered(resp.to_bin(), None)));
                }
            }
        }
    }
}
