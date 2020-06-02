use amethyst::{
    core::{Transform},
};
use serde::{Serialize, Deserialize};
use crate::components::{Item};

#[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ItemEvent {
    NewItem(Item, Transform),
}
