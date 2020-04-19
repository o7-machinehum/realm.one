use crate::map::Room;
use std::collections::HashMap;

// MapList for the server
pub struct MapList {
    pub list: HashMap<String, Room>,
    maps: Vec<String>,
}

impl Default for MapList {
    fn default() -> Self {
        MapList::new() 
    }
}

impl MapList {
    pub fn new() -> Self {
        Self {
            list: HashMap::<String, Room>::new(),
            maps: Vec::<String>::new(),
        }
    }
    
    pub fn add(&mut self, file_name: String) {
        self.maps.push(file_name.clone()); 
        self.list.insert(file_name.clone(), Room::new(file_name, true));
    }
    
    pub fn get(&self, file_name: &String) -> Option<&Room> {
        self.list.get(file_name)
    }

    fn get_rooms(&self) -> Vec<String> {
        let mut maps = Vec::<String>::new();

        for map in self.maps.iter() {
            maps.push(map.clone()); 
        }
        maps
    }
}
