use crate::network;
use crate::network::Pack;
use log::info;

use std::{
    fs::File,
    io::BufReader,
    path::Path,
};

use std::io::Read;

fn load_map() -> Option<Pack> {
    info!("Loading the map!");
    None 
}

pub fn handle(bin: Vec<u8>) -> Option<Pack> {
    let pk = network::Pack::from_bin(bin);
    info!("{:?}", pk);

    match pk.cmd {
        network::Cmd::Nothing       => None,
        network::Cmd::TransferMap   => load_map(), 
        network::Cmd::Connect       => None,
        network::Cmd::CreateMonster => None,
    }
}
