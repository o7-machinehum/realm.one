use amethyst::{
    core::{bundle::SystemBundle},
    core::{SystemDesc},
    ecs::{System, SystemData, World, DispatcherBuilder},
    shrev::{EventChannel, ReaderId}, 
    network::simulation::{NetworkSimulationEvent, NetworkSimulationTime, TransportResource}, 
    ecs,
    Result, 
};
use log::{info, error};

use crate::{
    components::Item,
    network::Pack,
    resources::{SpritesContainer},
};

use std::{
    thread,
    net::{TcpStream, TcpListener, Shutdown},
    io::{Read, Write},
    str::from_utf8,
    sync::mpsc,
};

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
        
        WalletSystem::new()
    }
}

pub struct WalletSystem { 
    up: bool,
}

impl WalletSystem {
    pub fn new() -> Self {

        Self { 
            up: false,
        }
    }
}

impl<'s> System<'s> for WalletSystem {
    type SystemData = (
        ecs::Read<'s, SpritesContainer>,
    );
    
    fn run(&mut self, (sprites): Self::SystemData) {
        // Just do this once.
        if !self.up {
            thread::spawn(|| {
                listen_client()        
            });
            self.up = true;
        }
    }
}

fn listen_client() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    info!("Server listening on port 3333");
    // match listener.accept() {
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            },
            Err(e) => {
                /* connection failed */
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 4098]; // buffer size 
    while match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                let msg = from_utf8(&data[0..size]).unwrap().to_string(); 
                info!("{:?}", msg);
                let item = Item::new(msg);
                info!("{:?}", item);
            }
            true
        },
        Err(_) => {
            info!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
