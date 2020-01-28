use serde::{Serialize, Deserialize};
use crate::components::{Orientation, PlayerComponent};

#[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Action {
    Nothing,
    Move(Orientation),
    Attack(u32),
    UseItem(u32),
}

pub fn verify(act: Action, mut player: &PlayerComponent) -> bool {

    true 
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct PlayerAction {
//     id: IdNum,
//     act: Action,
// }
// 
// impl PlayerAction {
//     pub fn new(id: u32, act: Action) -> Self {
//         let id = IdNum::new(id); 
//         Self {
//             id,
//             act,
//         }
//     }
// }
// 
// impl Component for PlayerAction {
//     type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
// }
