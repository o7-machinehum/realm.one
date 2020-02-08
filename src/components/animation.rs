use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

pub struct SimpleAnimation {
    pub start_sprite_index: usize, 
    pub frames: usize,
    pub current_frame: usize,
    pub time_per_frame: f32,
    pub elapsed_time: f32,
}

impl SimpleAnimation {
    pub fn new(start_sprite_index: usize, frames: usize, time_per_frame: f32)
        -> SimpleAnimation
{
        SimpleAnimation {
            start_sprite_index: start_sprite_index,
            frames: frames,
            current_frame: 0,
            time_per_frame: time_per_frame,
            elapsed_time: 0.0,
        }
    }
}

impl Component for SimpleAnimation {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
