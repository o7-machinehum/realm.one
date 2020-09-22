use amethyst::{
    core::{{bundle::SystemBundle}, Transform, SystemDesc},
    ecs::{System, SystemData, World, DispatcherBuilder},
    renderer::SpriteRender,
    ecs,
    Result, 
};
use log::{info, warn};

use crate::{
    components::{Item, SyncComponent},
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
        ecs::WriteStorage<'s, SyncComponent>,
        ecs::WriteStorage<'s, Transform>,
        ecs::Write<'s, Inventory>,
        ecs::Entities<'s>,
    );
    
    fn run(&mut self, (sc, item_res, mut renders, mut item_comp, mut sync, mut transforms, mut inventory, entities): Self::SystemData) {
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
                                .with(SyncComponent::Item, &mut sync)
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
            Err(_e) => (),
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
    let mut raw_data = [0 as u8; 4098]; // buffer size 
    let mut ptr: usize = 0;
    let mut item_size: usize = 0;
    let mut item_data = String::new(); 
    
    while match stream.read(&mut raw_data) {
        Ok(tcp_pack_size) => {
            while ptr < tcp_pack_size {
                info!("Size: {}", tcp_pack_size);
                // If we need to get the new item size
                if item_size == 0 {
                    item_size = u32::from_le_bytes(raw_data[ptr..ptr+4].try_into().unwrap()) as usize;
                    info!("Pointer: {}", ptr);
                    info!("Item String Size: {}", item_size);
                    ptr += 4;
                }
                
                // We don't have enough data. We need another TCP packet
                if tcp_pack_size < (ptr + item_size) {
                    // Push the entire packet to the string
                    item_data.push_str(&from_utf8(&raw_data[ptr..tcp_pack_size]).unwrap().to_string()); 
                    item_size -= tcp_pack_size;
                    break
                }

                else {
                    // We have a full item
                    item_data.push_str(&from_utf8(&raw_data[ptr..(item_size+ptr)]).unwrap().to_string()); 
                    let item = Item::new(item_data.clone());  // Create the item
                    item_data.clear();
                    tx.send(item).unwrap();
                    ptr += item_size;
                    item_size = 0;
                }
            }
            ptr = 0;
            true
        },
        Err(_) => {
            info!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}
