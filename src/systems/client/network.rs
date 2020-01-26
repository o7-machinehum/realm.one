use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, Time, bundle::SystemBundle},
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Read, Join, System, SystemData, World, Write, WriteStorage, DispatcherBuilder},
    shrev::{EventChannel, ReaderId}, 
    network::simulation::{DeliveryRequirement, UrgencyRequirement, NetworkSimulationEvent, TransportResource, NetworkSimulationTime},
    Result, 
};
use log::{info, error};
use crate::network;
use crate::constants;
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
            &[],); Ok(())
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
        let server_addr = constants::SERVER_IP.parse().unwrap();
        
        for frame in sim_time.sim_frames_to_run() {
            if !status.connected {
                let packet2 = Pack::new(Cmd::Connect("pubkey or some shit".to_string()), 0, None);  
                status.connected = true;
                net.send(server_addr, &packet2.to_bin());
            }
            else {
                for resp in io.o.pop() {
                    net.send_with_requirements(server_addr, &resp.to_bin(), DeliveryRequirement::ReliableSequenced(None), UrgencyRequirement::OnTick);
                }
            }
        }
        
        // Incoming packets
        for event in channel.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    io.i.push(Pack::from_bin(payload.to_vec())); // Add the pack to the IO vector
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
