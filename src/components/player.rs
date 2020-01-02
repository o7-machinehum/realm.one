use amethyst::{
    assets::Handle,
    renderer::SpriteSheet,
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};
use std::time::Instant;
use crate::character_sprites::Orientation;

pub struct PlayerComponent {
    pub x: f32,
    pub y: f32,
    pub orientation: Orientation,
    // Temporary solution, I am not convinced that it's the right place to store sprite sheet
    // handles.
    pub spritesheet_handle: Handle<SpriteSheet>,
    pub last_movement_instant: Instant,
}

impl PlayerComponent {
    pub fn new( x: f32, y: f32, spritesheet_handle: Handle<SpriteSheet> ) -> PlayerComponent {
        PlayerComponent {
            x,
            y, 
            orientation: Orientation::South,
            spritesheet_handle,
            last_movement_instant: Instant::now(),
        }
    }
}

impl Component for PlayerComponent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

