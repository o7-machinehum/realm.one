use amethyst::{
    core::{bundle::SystemBundle},
    core::{SystemDesc},
    ecs::{Read, System, SystemData, World, Write, DispatcherBuilder},
    shrev::{EventChannel, ReaderId}, 
    network::simulation::{NetworkSimulationEvent, NetworkSimulationTime, TransportResource}, 
    Result, 
};
use log::{info, error};

use crate::network::{Pack, Cmd, Dest};
use crate::resources::{AppConfig};
use crate::systems::client::{LifeformEvent, PlayerEvent, MapEvent};

pub struct WalletSystemBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for WalletSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            WalletSystemDesc::default().build(world),
            "client_wallet_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct WalletSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, WalletSystem> for WalletSystemDesc {
    fn build(self, world: &mut World) -> WalletSystem {
        <WalletSystem as System<'_>>::SystemData::setup(world);
        let net_reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();
        let packs_reader = world
            .fetch_mut::<EventChannel<Pack>>()
            .register_reader();
        
        WalletSystem::new(net_reader, packs_reader)
    }
}

pub struct WalletSystem {
    net_reader: ReaderId<NetworkSimulationEvent>,
    packs_reader: ReaderId<Pack>,
    connected: bool,
}

impl WalletSystem {
    pub fn new(net_reader: ReaderId<NetworkSimulationEvent>, packs_reader: ReaderId<Pack>) -> Self {
        Self { 
            net_reader,
            packs_reader,
            connected: false,
        }
    }
}

impl<'a> System<'a> for WalletSystem {
    type SystemData = (
        Read<'a, EventChannel<Pack>>,
        Write<'a, EventChannel<LifeformEvent>>,
        Write<'a, EventChannel<PlayerEvent>>,
        Write<'a, EventChannel<MapEvent>>,
        Read<'a, NetworkSimulationTime>,
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
        Read<'a, AppConfig>,
    );
    fn run(&mut self, (in_packs, mut lf_events, mut pl_events, mut map_events, sim_time, mut net, channel, conf): Self::SystemData) {

        // Incoming packets from the wallet
        let mut packs = Vec::<Pack>::new();
        for event in channel.read(&mut self.net_reader) {
            match event {
                NetworkSimulationEvent::Message(_addr, payload) => {
                    // info!("Payload: {:?}", payload);
                    if *payload != b"ok".to_vec() {
                        let pl =  Pack::from_bin(payload.to_vec());
                        // info!("Payload: {:?}", pl);
                        packs.push(pl);
                    }
                }
                NetworkSimulationEvent::Connect(addr) => info!("New wallet connection: {}", addr),
                NetworkSimulationEvent::Disconnect(addr) => {
                    info!("Wallet Disconnected: {}", addr);
                }
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                NetworkSimulationEvent::SendError(e, msg) => {
                    error!("Send Error: {:?}, {:?}", e, msg);
                }
                _ => {}
            }
        }
        
        for pack in packs.pop() {
            match pack.cmd {
                Cmd::UpdatePlayer(pl) => lf_events.single_write(LifeformEvent::UpdatePlayer(pl)),
                Cmd::RemovePlayer(u64) => lf_events.single_write(LifeformEvent::RemovePlayer(u64)),
                Cmd::InsertPlayer(pl) => pl_events.single_write(PlayerEvent::InsertPlayer(pl)),
                Cmd::InsertPlayer1(pl) => pl_events.single_write(PlayerEvent::InsertPlayer1(pl)),
                Cmd::TransferMap(map) => map_events.single_write(MapEvent::TransferMap(map)),
                _ => ()
            }
        }
    }
}
