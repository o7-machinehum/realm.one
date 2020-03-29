use amethyst::{
    prelude::*,
};

use crate::resources::{IO, AppConfig, MapList, LifeformList, LifeformUID};
use crate::components::{LifeformComponent};

pub struct ServerState{
    pub config: AppConfig,
}

impl SimpleState for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let io = IO::new();
        
        let mut uid = LifeformUID::new();

        // Load in all the maps
        let mut maps = MapList::new();
        maps.add("resources/maps/town.tmx".to_string());
        
        let mut lifeforms = LifeformList::new();
        
        // Insert all monsters into the lifeformlist
        for map in maps.list.values() {
            for monster in &map.monsters {
                let lf = LifeformComponent::new_monster(uid.add(), monster, map.name.clone());
                lifeforms.add(lf)
            }
        }

        world.insert(self.config.clone());
        world.insert(io);
        world.insert(maps);
        world.insert(lifeforms);
        world.insert(uid);
    }
}
