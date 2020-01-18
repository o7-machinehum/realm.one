use crate::network::{Pack, Cmd};
use log::info;

use std::{
    io::BufReader,
    path::Path,
};

use crate::events::{Events};
use stringreader::StringReader;
use crate::components::PlayerInfo;

fn create_player(id: u32, player: PlayerInfo) -> (Option<Pack>, Option<Events>) {
    info!("Insering Player id: {}, name: {} into the world", id, player.name);  
    // Add PlayerInfo to the big vector! 
    (None, None)
}

fn load_map(map_name: String, map_data: String) -> (Option<Pack>, Option<Events>) {
    info!("Loading the map: {}!", map_name);
    
    let streader = StringReader::new(&map_data);     // Make a buffer
    let reader = BufReader::new(streader);
    let map =  tiled::parse_with_path(reader, &Path::new("resources/sprites/master16.tsx")).unwrap();
    
    (None, Some(Events::NewMap(map)))
}

pub fn handle(bin: Vec<u8> ) -> (Option<Pack>, Option<Events>) {
    let pk = Pack::from_bin(bin);
    let id = pk.id;
    info!("{:?}", pk);

    match pk.cmd {
        Cmd::Nothing                   => (None, None),
        Cmd::TransferMap(name, data)   => load_map(name, data), 
        Cmd::Connect(..)               => (None, None),
        Cmd::CreatePlayer(pl)          => create_player(id, pl),
    }
}
