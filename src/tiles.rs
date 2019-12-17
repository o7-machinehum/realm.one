extern crate tiled;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use tiled::{parse_with_path, Map};

pub struct Room {
    room_name: String,
    map: tiled::Map,
}

impl Room {
    pub fn new(fname: String) -> Self{
        let file = File::open(&Path::new(&fname)).unwrap();
        let pth = &Path::new("resources/sprites/basictiles.tsx");
        println!("Opened file");
    	let reader = BufReader::new(file);
        
        Self{
            room_name: fname,
            map: parse_with_path(reader, pth).unwrap(), 
        }
    }
    pub fn getInfo(&self){
    	println!("{:?}", &self.map);
    	println!("{:?}", &self.map.get_tileset_by_gid(22));
    }
}
