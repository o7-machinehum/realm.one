use amethyst::{
    prelude::*,
};

pub struct ServerState;

impl SimpleState for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // Load in all the rooms

    }
}
