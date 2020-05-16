use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

/// Item component
#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Item {
  ItemName: String, 
  ItemClass: String, 
  Nuance: String,
  Owner: String,
  OriginWorld: String,
  GenesisTime: String,
  Stake: String,
  Hash: String, 
}

impl Item {
    pub fn new(item_string: String) -> Self {
        serde_json::from_str(&item_string).unwrap()
    }
}

impl Component for Item {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
