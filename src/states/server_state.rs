use amethyst::{
    prelude::*,
};

use crate::network::{IO};

pub struct ServerState;

impl SimpleState for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let io = IO::new();
        
        // let mut player_list = PlayerList{ list: Vec::new() };

        // let player1_info = PlayerInfo {
        //     id: 0,
        //     modified: true,
        //     act: PlayerAction::new(0, Action::Nothing),
        //     name: "Turnip".to_string(),
        //     room: "Room1".to_string(), 
        //     x: 8.0,        
        //     y: 8.0, 
        //     no: 318,        
        //     ea: 306, 
        //     so: 282,
        //     we: 294, 
        // };

        // let player2_info = PlayerInfo {
        //     id: 1,
        //     modified: true,
        //     act: PlayerAction::new(0, Action::Nothing),
        //     name: "Turnip".to_string(),
        //     room: "Room1".to_string(), 
        //     x: 5.0*8.0,        
        //     y: 5.0*8.0, 
        //     no: 318,        
        //     ea: 306, 
        //     so: 282,
        //     we: 294, 
        // };

        // player_list.list.push(player1_info);
        // player_list.list.push(player2_info);
        // world.insert(player_list);
        world.insert(io);
    }
}
