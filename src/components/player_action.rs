use serde::{Serialize, Deserialize};
use crate::components::{Orientation};

#[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    Nothing,
    Move(Orientation),
    Attack(u32),
    UseItem(u32),
}
