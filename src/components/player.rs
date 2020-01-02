use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};
use std::time::Instant;


pub struct PlayerComponent {
    pub x: f32,
    pub y: f32,
    pub last_movement_instant: Instant,
}

impl PlayerComponent {
    pub fn new( x: f32, y: f32 ) -> PlayerComponent {
        PlayerComponent {
            x,
            y, 
            last_movement_instant: Instant::now(),
        }
    }
}

impl Component for PlayerComponent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

