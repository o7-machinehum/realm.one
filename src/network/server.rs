use crate::network;
use crate::network::Pack;
use log::info;

use std::{
    fs::File,
};

use std::io::Read;

/// Send the map to the client
fn welcome() -> Option<Pack> {
    info!("Player Connected, sending map!");
    let fname = "resources/maps/townCompress2.tmx";
    let mut file = File::open(&fname.to_string()).expect("Unable to open map file"); 
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to convert to string");
    Some(Pack::send_tmx(fname.to_string(), contents.to_string()))
}

pub fn handle(bin: Vec<u8>) -> Option<Pack> {
    let pk = network::Pack::from_bin(bin);
    info!("{:?}", pk);

    match pk.cmd {
        network::Cmd::Nothing       => None,
        network::Cmd::TransferMap   => None, 
        network::Cmd::Connect       => welcome(),
        network::Cmd::CreateMonster => None,
        network::Cmd::CreatePlayer  => None,
    }
    // None
}
