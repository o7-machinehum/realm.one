use std::collections::HashMap;
use std::{fs::File, io::BufReader, path::Path};
use log::info;

extern crate tiled;

/// Hashmap of all the items in the game
pub struct Items {
    pub items: HashMap<String, u32>,
}

impl Default for Items {
    fn default() -> Self {
        Items::new()
    }
}

impl Items {
    pub fn new() -> Self {
        let file = File::open(&Path::new("resources/sprites/master16.tsx")).unwrap();
        let reader = BufReader::new(file);
        let tileset = tiled::parse_tileset(reader, 1).unwrap();

        for tile in tileset {
            match tile.properties {
                Some(prop) =>  
        }
        info!("{:?}", tileset);

        Self {
            items: HashMap::<String, u32>::new(),
        }
    }
}
