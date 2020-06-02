use amethyst::{
    core::{{bundle::SystemBundle}, Transform, SystemDesc},
    ecs::{System, SystemData, World, DispatcherBuilder, WriteStorage},
    renderer::SpriteRender,
    ecs,
    Result, 
};
use log::{info};

use crate::{
    components::Item,
    resources::{SpritesContainer, Inventory},
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
        ecs::WriteStorage<'s, SpriteRender>,
        ecs::WriteStorage<'s, Item>,
        ecs::WriteStorage<'s, Transform>,
        ecs::Write<'s, Inventory>,
        ecs::Entities<'s>,
    );
    
    fn run(&mut self, (sc, mut renders, mut items, mut transforms, mut inventory, entities): Self::SystemData) {
        // Just do this once.
        if !self.up {
            let (tx, rx) = mpsc::channel();
            self.rx = Some(rx);
            thread::spawn(move|| {
                listen_client(tx)
            });
            self.up = true;
        }
        
        // Get all the items from the thread
        // and stick them into the ECS system
        match self.rx.as_ref().unwrap().try_recv() {
            Ok(item) => {
                info!("New Item: {:?}", item);
                match inventory.take() {
                    Some(spot) => {
                        entities
                            .build_entity()
                            .with(item, &mut items)
                            .with(spot, &mut transforms)
                            .with(sc.sprites[963].clone(), &mut renders)
                            .build();
                    },
                    None => info!("Inventory is full!"),
                }
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

fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) +
    ((array[1] as u32) << 16) +
    ((array[2] as u32) <<  8) +
    ((array[3] as u32) <<  0)
}

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<Item>) {
    let mut data = [0 as u8; 4098]; // buffer size 
    let mut ptr: usize = 0;
    let mut pack_size: usize = 0;

    while match stream.read(&mut data) {
        Ok(size) => {
            info!("Size: {}", size);
            while ptr < size {
                // First four bytes are the size of the pack 
                if pack_size == 0 {
                    // pack_size = as_u32_be(&data[0..4]) as usize;
                    Pack_size = u32::from_be_bytes(&data[0..4]);
                    info!("Pack Size: {}", pack_size);
                    ptr += 4;
                    if size < pack_size {
                        info!("Shit, this shouldn't have happend. We got half a pack");
                    }
                    else {
                        let msg = from_utf8(&data[ptr..ptr+pack_size]).unwrap().to_string(); 
                        info!("{:?}", msg);
                        let item = Item::new(msg);
                        tx.send(item).unwrap();
                        ptr += pack_size;
                        info!("Ptr: {}", ptr);
                        pack_size = 0;
                    }
                }
                // info!("{:?}", item);
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
