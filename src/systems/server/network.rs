use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, Write, System, SystemData, World, WriteStorage},
    network::*,
};
use crate::network;
use log::info;
use crate::network::{Pack, IO, Cmd};

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct ServerSystem; 

impl<'a> System<'a> for ServerSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<Vec<u8>>>,
        WriteStorage<'a, network::Reader>,
        Write <'a, IO>,
        Entities<'a>,
    );

    fn run(&mut self, (mut connections, mut readers, mut io, entities): Self::SystemData) {
        for (e, connection) in (&entities, &mut connections).join() {
            let reader = readers
                .entry(e)
                .expect("Cannot get reader")
                .or_insert_with(|| network::Reader(connection.register_reader()));
            
            // let _pk_out = Vec::<Pack>::new();

            // Command / Responce below  
            let mut client_disconnected = false;
            let mut client_connection = false;
            for ev in connection.received_events(&mut reader.0) {
                info!("{:?}", connection.state);
                if connection.state == ConnectionState::Connecting {
                    client_connection = true;
                    // break
                }

                // Get Pack 
                info!("{:?}", ev);
                let pack = match ev {
                    NetEvent::Packet(packet) => Some(packet),
                    NetEvent::Connected(addr) => {
                        info!("Client: {} is connected!", addr); 
                        None
                    },
                    NetEvent::Disconnected(addr) => {
                        info!("Client: {} is Disconnected!", addr); 
                        client_disconnected = true;
                        None 
                    },
                    _ => None
                };

                match pack {
                    Some(pack) => {
                        let mut pk = Pack::from_bin(pack.content().to_vec());
                        pk.update_ip(connection.target_addr); // RTS
                        io.i.push(pk);    
                    }
                    None => (),
                }
            }
            
            // if client_connection {
            //     let p = Pack::new(Cmd::Ping, 0, None);
            //     connection.queue(NetEvent::Packet(NetPacket::unreliable(p.to_bin())));
            // }
            
            if client_disconnected {
                info!("Client Disconnects");
                entities
                    .delete(e)
                    .expect("Cannot delete connection from world!");
            }
            
            // If there is a ip in the ip field send the message to that clinet, if not
            // sent to all of the client
            
            for element in io.o.pop() {
                match &element.ip() {
                    Some(ip) => {
                        if *ip == connection.target_addr {
                            connection.queue(NetEvent::Packet(NetPacket::reliable_ordered(element.to_bin(), None)));
                        } 
                        else {
                            io.o.push(element);
                        }
                    },
                    None => connection.queue(NetEvent::Packet(NetPacket::reliable_ordered(element.to_bin(), None))),
                }
            }
        }
    }
}
