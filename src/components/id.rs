use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdNum{
    id: u32,     // Server ID
}

impl IdNum {
    pub fn new(id: u32) -> Self {
        Self{ id }
    }
}

impl Component for IdNum {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
