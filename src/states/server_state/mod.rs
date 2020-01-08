use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteSheet},
    window::ScreenDimensions,
};

pub struct ServerState;

impl SimpleState for ServerState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {}
}
