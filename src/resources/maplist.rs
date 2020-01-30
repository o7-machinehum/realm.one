use crate::map::Room;

// MapList for the server
pub struct MapList {
    pub list: Vec<Room>,
}

impl Default for MapList {
    fn default() -> Self {
        Self{ list: Vec::new(), } 
    }
}

impl MapList {
    pub fn add(&mut self, file_name: String) {
        self.list.push(Room::new(file_name));
    }
}

