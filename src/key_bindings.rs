use crate::components::Orientation;
use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {
    Move(Orientation),
    Melee,
    TypingMode,
}

impl Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct MovementBindingTypes;

impl BindingTypes for MovementBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
