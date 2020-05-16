use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

/// Item component
pub struct Item {
    item_string: String,
}

impl Item {
    pub fn new(item_string: String) -> Self {
        Self {
            item_string,
        }
    }
}

impl Component for Item {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
