use crate::network;
use crate::network::Pack;
use log::info;

use std::{
    fs::File,
    io::BufReader,
    path::Path,
};

use std::io::Read;

/// Send the map to the client
fn welcome() -> Option<Pack> {
    info!("Player Connected, sending map!");
    let fname = "resources/sprites/townCompress.tmx";
    let mut file = File::open(&fname.to_string()).expect("Unable to open map file"); 
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    // Some(Pack::send_tmx("map1".to_string(), "hi".to_string()))
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
    }
    // None
}
