use amethyst::{
    core::{bundle::SystemBundle},
    core::{SystemDesc},
    ecs::{Read, System, SystemData, World, Write, DispatcherBuilder},
    shrev::{EventChannel, ReaderId}, 
    network::simulation::{tcp::TcpNetworkBundle, NetworkSimulationEvent, NetworkSimulationTime, TransportResource}, 
    Result, 
};
use log::{info, error};

use crate::network::{Pack, Cmd};
use crate::resources::{ClientStatus, IO, AppConfig};

/// A simple system that sends a ton of messages to all connections.
/// In this case, only the server is connected.
pub struct TcpSystem;

impl TcpSystem {
    pub fn new() -> Self {
        TcpSystem {}
    }
}

impl<'a> System<'a> for TcpSystem {
    type SystemData = (
        Write<'a, ClientStatus>, 
        Read<'a, NetworkSimulationTime>,
        Write<'a, TransportResource>,
        // Read<'a, EventChannel<NetworkSimulationEvent>>,
        Write <'a, IO>,
        Read<'a, AppConfig>,

    );
    fn run(&mut self, (mut status, sim_time, mut net, /* channel, */ mut io, conf): Self::SystemData) {
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
