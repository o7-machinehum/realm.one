use crate::map::Room;
use std::collections::HashMap;

// MapList for the server
pub struct MapList {
    list: HashMap<String, Room>,
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
        }
    }
    
    pub fn add(&mut self, file_name: String) {
        self.list.insert(file_name.clone(), Room::new(file_name));
    }
    
    pub fn get(&self, file_name: &String) -> Option<&Room> {
        self.list.get(file_name)
    }
    
    pub fn get_mut(&mut self, file_name: &String) -> Option<&mut Room> {
        self.list.get_mut(file_name)
    }

}

