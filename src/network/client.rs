use crate::network::{Pack, Cmd};
use log::info;

use std::{
    io::BufReader,
    path::Path,
};

use crate::events::{Events};
use stringreader::StringReader;
use crate::components::PlayerInfo;

fn create_player(mut pk: Pack) -> (Option<Pack>, Option<Events>) {
    // let player: PlayerInfo = pk.get_struct();

    //(None, Some(Events::CreatePlayer(player)))
    (None, None)
}

fn load_map(map_name: String, map_data: String) -> (Option<Pack>, Option<Events>) {
    info!("Loading the map!");
    
    let streader = StringReader::new(&map_data);     // Make a buffer
    let reader = BufReader::new(streader);
    let map =  tiled::parse_with_path(reader, &Path::new("resources/sprites/master16.tsx")).unwrap();
    
    (None, Some(Events::NewMap(map)))
}

pub fn handle(bin: Vec<u8> ) -> (Option<Pack>, Option<Events>) {
    let pk = Pack::from_bin(bin);
    info!("{:?}", pk);

    match pk.cmd {
        Cmd::Nothing                   => (None, None),
        Cmd::TransferMap(name, data)   => load_map(name, data), 
        Cmd::Connect(proof)            => (None, None),
        Cmd::CreatePlayer              => create_player(pk),
    }
}
