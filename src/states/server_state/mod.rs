use amethyst::{
    prelude::*,
};

pub struct ServerState;

impl SimpleState for ServerState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
