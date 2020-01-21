use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, System, SystemData, World, Write, WriteStorage},
    network::*,
};
use log::info;
use crate::network;
use crate::network::{Pack, Cmd, IO};
use crate::resources::ClientStatus;
use crate::map::Room;
use crate::components::PlayerList;

/// A simple system that sends a ton of messages to all connections.
/// In this case, only the server is connected.
#[derive(SystemDesc)]
pub struct ClientSystem;

impl<'a> System<'a> for ClientSystem {
    type SystemData = (
        Write<'a, ClientStatus>, 
        WriteStorage<'a, NetConnection<Vec::<u8>>>,
        WriteStorage<'a, network::Reader>,
        Write<'a, Room>,
        Write<'a, PlayerList>,
        Write <'a, IO>,
        Entities<'a>,
    );

    fn run(&mut self, (mut status, mut connections, mut readers, mut room, mut p_list, mut io, entities): Self::SystemData) {
        for (e, connection) in (&entities, &mut connections).join() {
            let reader = readers
                .entry(e)
                .expect("Cannot get reader")
                .or_insert_with(|| network::Reader(connection.register_reader()));
            
            if !status.connected {
                let mut packet = Pack::new(Cmd::Connect("pubkey or some shit".to_string()), 0, None);  
                status.connected = true;
                io.O.push(packet); 
            }
            
            else {
                for ev in connection.received_events(&mut reader.0) {
                    // Get Pack 
                    let pack = match ev {
                        NetEvent::Packet(packet) => Some(packet),
                        NetEvent::Connected(_addr) => None,
                        NetEvent::Disconnected(_addr) => None,
                        _ => None
                    };
                    
                    match pack {
                        Some(pack) => {
                            // info!("{:?}", pack.content()); 
                            info!("Adding Something"); 
                            io.I.push(Pack::from_bin(pack.content().to_vec())); // Add the pack to the IO vector
                        },
                        None => (),
                    }
                }

                // Respond
                // TODO: There's this member that can be used for vectors. Should use that.
                for mut resp in io.O.pop() {
                    info!("Sending"); 
                    connection.queue(NetEvent::Packet(NetPacket::reliable_ordered(resp.to_bin(), None)));
                }
            }
        }
    }
}
