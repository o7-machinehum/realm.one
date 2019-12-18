extern crate tiled;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use log::info;

use tiled::parse;

pub struct Room {
    name: String,
    pub current: tiled::Map,
    pub tiles: Vec<i32>,
}

impl Room {
    fn countTiles(mp: &tiled::Map) -> Vec<i32>{
        let mut v: Vec<i32> = Vec::new();
        for sets in &mp.tilesets{
            v.push((sets.images[0].width / sets.tile_width as i32) * (sets.images[0].height / sets.tile_height as i32)) 
        }
        info!("Tiles in the images: {:?}", v);

        v
    }

    pub fn new(fname: String) -> Self{
        let file = File::open(&Path::new(&fname)).unwrap();
    	let reader = BufReader::new(file);
        let mp =  parse(reader).unwrap();

        info!("{:?}", mp);

        Self{
            name: fname,
            tiles: Room::countTiles(&mp), 
            current: mp,
        }
    }
}
