use std::collections::HashMap;
use std::{fs::File, io::BufReader, path::Path};
use log::{info, warn};
use tiled::PropertyValue::StringValue;

extern crate tiled;

/// Hashmap of all the items in the game
pub struct Items {
    pub items: HashMap<String, usize>,
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
        let mut items = HashMap::<String, usize>::new();

        for tile in tileset.tiles {
            match (tile.properties.get("item_name"), tile.properties.get("item_class")) {
                (Some(_name), Some(_class)) => {
                    match (_name, _class) {
                        (StringValue(_name), StringValue(_class)) => {
                            items.insert(_name.to_string(), tile.id as usize); 
                            // info!("{} {}", name, class);
                        },
                        _ => (),
                    }
                },
                (None, Some(_class))       => warn!("Item Class with no name!"),
                (Some(_name), None )       => warn!("Item Name with no class!"),
                (None, None)              => (),
            }
        }
        info!("{:?}", items);

        Self {
            items,
        }
    }
}
