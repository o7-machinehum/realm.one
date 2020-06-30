use amethyst::{
    core::{{bundle::SystemBundle}, Transform, SystemDesc},
    ecs::{System, SystemData, World, DispatcherBuilder, WriteStorage},
    renderer::SpriteRender,
    ecs,
    Result, 
};
use log::{info, warn};

use crate::{
    components::Item,
    resources::{SpritesContainer, Inventory, Items},
};

use std::{
    thread,
    net::{TcpStream, TcpListener, Shutdown},
    io::{Read},
    str::from_utf8,
    sync::mpsc,
    convert::TryInto,
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
        ecs::Read<'s, Items>,
        ecs::WriteStorage<'s, SpriteRender>,
        ecs::WriteStorage<'s, Item>,
        ecs::WriteStorage<'s, Transform>,
        ecs::Write<'s, Inventory>,
        ecs::Entities<'s>,
    );
    
    fn run(&mut self, (sc, item_res, mut renders, mut item_comp, mut transforms, mut inventory, entities): Self::SystemData) {
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
                if let Some(item_index) = item_res.items.get(&item.ItemName) {
                    match inventory.take() {
                        Some(spot) => {
                            entities
                                .build_entity()
                                .with(item, &mut item_comp)
                                .with(spot, &mut transforms)
                                .with(sc.sprites[*item_index].clone(), &mut renders)
                                .build();
                        },
                        None => info!("Inventory is full!"),
                    }
                }
                else {
                    warn!("Item has been sent that doesn't exist in game");
                }
            }
            Err(e) => (),
        }
    }
}

fn listen_client(tx_main: mpsc::Sender<Item>) {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    info!("Server listening on port 3333");
    match listener.accept() {
        Ok((stream, addr)) => {
            info!("New connection: {}", addr); 

            thread::spawn(move|| {
                // connection succeeded
                handle_client(stream, tx_main)
            });
        },
        Err(e) => {
            info!("{:?}", e);
            /* connection failed */
        }
    }

}

fn handle_client(mut stream: TcpStream, tx: mpsc::Sender<Item>) {
    let mut data = [0 as u8; 4098]; // buffer size 
    let mut ptr: usize = 0;
    let mut string_size: usize;

    while match stream.read(&mut data) {
        Ok(size) => {
            while ptr < size {
                string_size = u32::from_le_bytes(data[ptr..ptr+4].try_into().unwrap()) as usize;
                ptr += 4;

                if size < string_size {
                    info!("Shit, this shouldn't have happend. We got half a pack");
                }
                else {
                    let msg = from_utf8(&data[ptr..ptr+string_size]).unwrap().to_string(); 
                    let item = Item::new(msg);
                    // info!("Item: {:?}", item);
                    tx.send(item).unwrap();
                    ptr += string_size;
                }
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
