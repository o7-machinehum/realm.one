use amethyst::{
    prelude::*,
};

use crate::components::{PlayerList, PlayerAction, PlayerInfo};

pub struct ServerState;

impl SimpleState for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let mut player_list = PlayerList{ list: Vec::new() };
        
        let player1_info = PlayerInfo {
            id: 0,
            modified: true,
            act: PlayerAction::Nothing,
            name: "Turnip".to_string(),
            room: "Room1".to_string(), 
            x: 8.0,        
            y: 8.0, 
            no: 318,        
            ea: 306, 
            so: 282,
            we: 294, 
        };

        player_list.list.push(player1_info);
        world.insert(player_list);
    }
}
