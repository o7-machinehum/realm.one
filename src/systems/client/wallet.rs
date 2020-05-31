use amethyst::{
    core::{bundle::SystemBundle},
    core::{SystemDesc},
    ecs::{System, SystemData, World, DispatcherBuilder},
    ecs,
    Result, 
};
use log::{info};

use crate::{
    components::Item,
    resources::{SpritesContainer},
};

use std::{
    thread,
    net::{TcpStream, TcpListener, Shutdown},
    io::{Read},
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
    rx: Option<mpsc::Receiver<Item>>,
}

impl WalletSystem {
    pub fn new() -> Self {
        Self { 
            up: false,
            rx: None,
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
            let (tx, rx) = mpsc::channel();
            self.rx = Some(rx);
            thread::spawn(move|| {
                listen_client(tx)
            });
            self.up = true;
        }
        
        match self.rx.as_ref().unwrap().try_recv() {
                Ok(item) => {
                    info!("New Item: {:?}", item);
                }
                Err(e) => (),
            }
    }
}

fn listen_client(tx_main: mpsc::Sender<Item>) {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    info!("Server listening on port 3333");
    // match listener.accept() {
    let mut rx_list = Vec::<mpsc::Receiver<Item>>::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New connection: {}", stream.peer_addr().unwrap());
                let (tx, rx) = mpsc::channel();
                rx_list.push(rx);

                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream, tx)
                });
            },
            Err(e) => {
                info!("{:?}", e);
                /* connection failed */
            }
        }
        for rx in &rx_list {
            match rx.try_recv() {
                Ok(item) => {
                    info!("{:?}", item);
                    tx_main.send(item).unwrap();
                },
                Err(_e) => (),
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<Item>) {
    let mut data = [0 as u8; 4098]; // buffer size 
    while match stream.read(&mut data) {
        Ok(size) => {
            if size > 0 {
                let msg = from_utf8(&data[0..size]).unwrap().to_string(); 
                // info!("{:?}", msg.as_bytes());
                let item = Item::new(msg);
                // info!("{:?}", item);
                tx.send(item).unwrap();
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
