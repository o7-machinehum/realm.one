use crate::network::{Pack, Cmd};
use log::info;

use std::{
    fs::File,
};

use std::io::Read;

/// Send the map to the client
fn welcome(proof: String) -> Option<Pack> {
    info!("Player Connected, sending map!");
    let fname = "resources/maps/townCompress2.tmx";
    let mut file = File::open(&fname.to_string()).expect("Unable to open map file"); 
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to convert to string");
    Some(Pack::new(Cmd::TransferMap(fname.to_string(), contents.to_string()), 0))
}

pub fn handle(bin: Vec<u8>) -> Option<Pack> {
    let pk = Pack::from_bin(bin);
    info!("{:?}", pk);

    match pk.cmd {
        Cmd::Nothing              => None,
        Cmd::TransferMap(i, ii)   => None, 
        Cmd::Connect(proof)       => welcome(proof),
        Cmd::CreatePlayer         => None,
    }
    // None
}
