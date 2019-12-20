extern crate tiled;
use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};

pub struct Location {
    pub x: u32,
    y: u32,
    z: u32,
    rot: u32,
}

impl Component for Location{
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Location{
    pub fn new(x: u32, y: u32, z: u32) -> Self{
        Self{
            x,
            y,
            z,
            rot: 0,
        }
    }
    pub fn X(&self) -> u32{
        self.x
    }
}
