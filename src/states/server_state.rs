use amethyst::{
    prelude::*,
};

use crate::network::{IO};
use crate::map::{Room, MapList};
use crate::appconfig::{AppConfig};

pub struct ServerState{
    pub config: AppConfig,
}

impl SimpleState for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let io = IO::new();
        
        // Load in all the maps in the world
        let mut maps = MapList{ list: Vec::<Room>::new(), };
        maps.add("resources/maps/townCompress.tmx".to_string());
        maps.add("resources/maps/townCompress2.tmx".to_string());

        world.insert(self.config.clone());
        world.insert(io);
        world.insert(maps);
    }
}
