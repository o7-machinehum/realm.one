use serde::{Serialize, Deserialize};
use crate::components::{Orientation};
use crate::components::{Skins};

#[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    Nothing,
    Move(Orientation),
    Rotate(Orientation),
    Attack(u32),
    UseItem(u32),
    ChangeOutfit(Skins),
    Melee,
}
