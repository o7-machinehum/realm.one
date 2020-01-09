use amethyst::{
    core::{SystemDesc, Time},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, World, WriteStorage},
    network::*,
};
use log::info;
use crate::network::Pack;

/// A simple system that sends a ton of messages to all connections.
/// In this case, only the server is connected.
#[derive(SystemDesc)]
pub struct ClientSystem;

impl<'a> System<'a> for ClientSystem {
    type SystemData = (
        WriteStorage<'a, NetConnection<String>>,
        Read<'a, Time>
    );

    fn run(&mut self, (mut connections, time): Self::SystemData) {
        for conn in (&mut connections).join() {
            info!("Sending 10k messages.");
            for i in 0..500 {
                // let packet = NetEvent::Packet(NetPacket::unreliable(format!(
                //     "CL: frame:{},abs_time:{},c:{}",
                //     time.frame_number(),
                //     time.absolute_time_seconds(),
                //     i
                // )));
                
                let mut packet = Pack::pack_monster(1, 16.0, 16.0, 100.0, 32, "Evil Fucker".to_string());  
                conn.queue(NetEvent::Packet(NetPacket::unreliable(packet.to_string())));
            }
        }
    }
}
