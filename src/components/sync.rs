use amethyst::{
    //core::{Transform},
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};
use serde::{Serialize, Deserialize};

#[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SyncComponent {
    Item,
}

impl Component for SyncComponent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
