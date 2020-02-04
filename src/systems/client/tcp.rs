use amethyst::{
    core::{bundle::SystemBundle},
    core::{SystemDesc},
    ecs::{Read, System, SystemData, World, Write, DispatcherBuilder},
    shrev::{EventChannel, ReaderId}, 
    network::simulation::{tcp::{TcpNetworkBundle, TcpNetworkResource}, NetworkSimulationEvent, NetworkSimulationTime, TransportResource}, 
    Result, 
};
use log::{info, error};

use crate::network::{Pack, Cmd};
use crate::resources::{ClientStatus, IO, AppConfig};

pub struct TcpSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for TcpSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            TcpSystemDesc::default().build(world),
            "client_tcp_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct TcpSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, TcpSystem> for TcpSystemDesc {
    fn build(self, world: &mut World) -> TcpSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        // <TcpSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        // let reader = world
        //     .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
        //     .register_reader();
        // TcpSystem::new(reader)
        TcpSystem
    }
}

// pub struct TcpSystem {
//     reader: ReaderId<NetworkSimulationEvent>,
// }
// 
// impl TcpSystem {
//     pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
//         Self { 
//             reader,
//         }
//     }
// }

pub struct TcpSystem;

impl<'a> System<'a> for TcpSystem {
    type SystemData = (
        Write<'a, ClientStatus>, 
        Read<'a, NetworkSimulationTime>,
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
        Write <'a, IO>,
        Read<'a, AppConfig>,
    );
    fn run(&mut self, (mut status, sim_time, mut net, channel, mut io, conf): Self::SystemData) {
        if sim_time.should_send_message_now() {
            if !status.connected {
                let server_addr = "127.0.0.1:3457".parse().unwrap(); 
                info!("We are not connected, ready player 1");
                let proof = format!("{} 1580235330 SignatureHere", conf.player_name);
                let p = Pack::new(Cmd::Connect(proof.to_string()), 0, None);  
                net.send(server_addr, &p.to_bin());
                status.connected = true;
            }
            else {
                for resp in io.o.pop() {
                    net.send(conf.server_ip.parse().unwrap(), &resp.to_bin());
                }
            }
        }

        // Incoming packets
        // for event in channel.read(&mut self.reader) {
        //     match event {
        //         NetworkSimulationEvent::Message(_addr, payload) => {
        //             if *payload != b"ok".to_vec() {
        //                 let pl =  Pack::from_bin(payload.to_vec());
        //                 info!("Payload: {:?}", pl);
        //                 io.i.push(pl); // Add the pack to the IO vector
        //             }
        //         }
        //         NetworkSimulationEvent::Connect(addr) => info!("New client connection: {}", addr),
        //         NetworkSimulationEvent::Disconnect(addr) => {
        //             info!("Server Disconnected: {}", addr);
        //         }
        //         NetworkSimulationEvent::RecvError(e) => {
        //             error!("Recv Error: {:?}", e);
        //         }
        //         _ => {}
        //     }
        // }
    }
}
