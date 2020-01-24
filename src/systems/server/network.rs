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
use std::net::{SocketAddr};

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
    clients: Vec<SocketAddr>,
}

impl ServerSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { 
            reader,
            clients: Vec::<SocketAddr>::new(),
        }
    }
}

impl<'a> System<'a> for ServerSystem {
    type SystemData = (
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
        Write <'a, IO>,
    );

    fn run(&mut self, (mut net, channel, mut io): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("{}: {:?}", addr, payload);
                    let mut pk = Pack::from_bin(payload.to_vec());
                    pk.addr = Some(addr.clone());  // Update the client addr
                    // pk.addr = None;  // Update the client addr
                    io.i.push(pk);
                }
                NetworkSimulationEvent::Connect(addr) => {
                    info!("New client connection: {}", addr);
                    self.clients.push(*addr);
                } 
                NetworkSimulationEvent::Disconnect(addr) => {
                    info!("Client Disconnected: {}", addr);
                    self.clients.retain(|&x| x != *addr); 
                }
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                _ => {}
            }
        }
        
        // Send responces
        for resp in io.o.pop() {
            info!("{:?}", resp);
            match resp.addr {
                Some(addr) => {
                    info!("Sending to: {}", addr);
                    net.send(addr, &resp.to_bin());
                },
                // Broadcast message
                None => {
                    for addr in self.clients.clone() {
                        info!("Sending to: {}", addr);
                        net.send(addr, &resp.to_bin());
                    }
                }
            }
        }
    }
}
