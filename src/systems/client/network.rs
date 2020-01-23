use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, Time, bundle::SystemBundle},
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Read, Join, System, SystemData, World, Write, WriteStorage, DispatcherBuilder},
    shrev::{EventChannel, ReaderId}, 
    network::simulation::{udp::UdpNetworkBundle, NetworkSimulationEvent, TransportResource, NetworkSimulationTime},
    Result, 
};
use log::{info, error};
use crate::network;
use crate::network::{Pack, Cmd, IO};
use crate::resources::ClientStatus;
use crate::map::Room;
use crate::components::PlayerList;

/// A simple system that sends a ton of messages to all connections.
/// In this case, only the server is connected.
#[derive(Debug)]
pub struct ClientSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for ClientSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            ClientSystemDesc::default().build(world),
            "client_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ClientSystemDesc;

/// A simple system that receives a ton of network events.
impl<'a, 'b> SystemDesc<'a, 'b, ClientSystem> for ClientSystemDesc {
    fn build(self, world: &mut World) -> ClientSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <ClientSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();
        ClientSystem::new(reader)
    }
}

pub struct ClientSystem {
    reader: ReaderId<NetworkSimulationEvent>,
}

impl ClientSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
    }
}

impl<'a> System<'a> for ClientSystem {
    type SystemData = (
        Write<'a, ClientStatus>, 
        Read<'a, NetworkSimulationTime>,
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
        Write <'a, IO>,
    );

    fn run(&mut self, (mut status, sim_time, mut net, channel, mut io): Self::SystemData) {
        // Use method `sim_time.sim_frames_to_run()` to determine if the system should send a
        // message this frame. If, for example, the ECS frame rate is slower than the simulation
        // frame rate, this code block will run until it catches up with the expected simulation
        // frame number.
        let server_addr = "127.0.0.1:3456".parse().unwrap();
        
        for frame in sim_time.sim_frames_to_run() {
            if !status.connected {
                let packet2 = Pack::new(Cmd::Connect("pubkey or some shit".to_string()), 0, None);  
                status.connected = true;
                net.send(server_addr, &packet2.to_bin());
            }
            else {
                for resp in io.o.pop() {
                    net.send(server_addr, &resp.to_bin());
                }
            }
        }
        
        // Incoming packets
        for event in channel.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("{}: {:?}", addr, payload);
                    io.i.push(Pack::from_bin(payload.to_vec())); // Add the pack to the IO vector
                    info!("Made it.");
                }
                NetworkSimulationEvent::Connect(addr) => info!("New client connection: {}", addr),
                NetworkSimulationEvent::Disconnect(addr) => {
                    info!("Server Disconnected: {}", addr);
                }
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                _ => {}
            }
        } 
    }
}

/*
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

    fn run(&mut self, (mut status, mut connections, mut readers, _room, _p_list, mut io, entities): Self::SystemData) {
        for (e, connection) in (&entities, &mut connections).join() {
            let reader = readers
                .entry(e)
                .expect("Cannot get reader")
                .or_insert_with(|| network::Reader(connection.register_reader()));
            
            if !status.connected {
                let packet2 = Pack::new(Cmd::Connect("pubkey or some shit".to_string()), 0, None);  
                connection.queue(NetEvent::Packet(NetPacket::unreliable(packet2.to_bin())));
                status.connected = true;
            }
            
            else {
                for ev in connection.received_events(&mut reader.0) {
                    info!("{:?}", connection.state);
                    // Get Pack 
                    let pack = match ev {
                        NetEvent::Packet(packet) => Some(packet),
                        NetEvent::Connected(addr) => {
                            info!("Server Connected: {}", addr);
                            None 
                        },
                        NetEvent::Disconnected(addr) => {
                            info!("Server Disconnected: {}", addr);
                            None
                        }
                        _ => None
                    };
                
                    info!("{:?}", pack);
                    
                    match pack {
                        Some(pack) => {
                            // info!("{:?}", pack.content()); 
                            info!("Adding Something"); 
                            io.i.push(Pack::from_bin(pack.content().to_vec())); // Add the pack to the IO vector
                        },
                        None => (),
                    }
                }

                // Respond
                // TODO: There's this member that can be used for vectors. Should use that.
                for resp in io.o.pop() {
                    info!("{:?}", resp); 
                    connection.queue(NetEvent::Packet(NetPacket::reliable_sequenced(resp.to_bin(), None)));
                    info!("sent..."); 
                }
            }
        }
    }
}
*/
