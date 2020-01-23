use amethyst::{
    core::{SystemDesc, bundle::SystemBundle},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, Write, System, SystemData, World, WriteStorage, DispatcherBuilder},
    shrev::{EventChannel, ReaderId}, 
    network::simulation::{udp::UdpNetworkBundle, NetworkSimulationEvent, TransportResource, NetworkSimulationTime},
    Result,
};
use crate::network;
use log::{info, error};
use crate::network::{Pack, IO, Cmd};


#[derive(Debug)]
pub struct ServerSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ServerSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            ServerSystemDesc::default().build(world),
            "server_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ServerSystemDesc;

/// A simple system that receives a ton of network events.
impl<'a, 'b> SystemDesc<'a, 'b, ServerSystem> for ServerSystemDesc {
    fn build(self, world: &mut World) -> ServerSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <ServerSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();
        ServerSystem::new(reader)
    }
}

pub struct ServerSystem {
    reader: ReaderId<NetworkSimulationEvent>,
}

impl ServerSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
    }
}

impl<'a> System<'a> for ServerSystem {
    type SystemData = (
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
        Write <'a, IO>,
    );

    fn run(&mut self, (mut net, channel, mut io): Self::SystemData) {
        let client_addr = "127.0.0.1:3455".parse().unwrap();
        
        for event in channel.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("{}: {:?}", addr, payload);
                    let mut pk = Pack::from_bin(payload.to_vec());
                    io.i.push(pk);
                    // net.send(*addr, b"ok");
                }
                NetworkSimulationEvent::Connect(addr) => info!("New client connection: {}", addr),
                NetworkSimulationEvent::Disconnect(addr) => {
                    info!("Client Disconnected: {}", addr);
                }
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                _ => {}
            }
        }
        
        for resp in io.o.pop() {
            info!("{:?}", resp);
            net.send(client_addr, &resp.to_bin());
        }
    }
}

/*
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
*/
