use crate::network;
use crate::network::Pack;
use log::info;
use crate::map::{Room, Adj};

use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, System, SystemData, World, Write, WriteStorage},
    network::*,
    shrev::EventChannel
};

use std::{
    fs::File,
    io::BufReader,
    path::Path,
};

use std::io::Read;
use crate::events::{Events};
use stringreader::StringReader;

fn load_map(mut pk: Pack, entities: &Entities) -> (Option<Pack>, Option<Events>) {
    info!("Loading the map!");
    
    let string = pk.strings.pop().unwrap();        // Get the string
    let mut streader = StringReader::new(&string); // Make a buffer
    let reader = BufReader::new(streader);
    let map =  tiled::parse_with_path(reader, &Path::new("resources/sprites/master16.tsx")).unwrap();
    
    (None, Some(Events::NewMap(map)))
}

pub fn handle(bin: Vec<u8>, entities: &Entities ) -> (Option<Pack>, Option<Events>) {
    let pk = network::Pack::from_bin(bin);
    info!("{:?}", pk);

    match pk.cmd {
        network::Cmd::Nothing       => (None, None),
        network::Cmd::TransferMap   => load_map(pk, entities), 
        network::Cmd::Connect       => (None, None),
        network::Cmd::CreateMonster => (None, None),
    }
}
