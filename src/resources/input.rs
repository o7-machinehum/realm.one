use crate::components::Orientation;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Inputs {
    Move(Orientation),
    Melee,
    TypingMode,
    TypedData(String),
}
impl Display for Inputs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub struct Input {
    list: Vec<Inputs>,
}
impl Default for Input {
    fn default() -> Self {
        Input::new()
    }
}
impl Input {
    pub fn new() -> Self {
        Self {
            list: Vec::<Inputs>::new(),
        }
    }
    pub fn add(&mut self, input: Inputs) {
        self.list.push(input);
    }
    pub fn get(&mut self) -> Option<Inputs> {
        self.list.pop()
    }
}
