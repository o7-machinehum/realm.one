use crate::components::Orientation;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Command {
    Move(Orientation),
    Melee,
    TypingMode,
    TypedData(String),
}
impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub struct CommandQueue {
    list: Vec<Command>,
}
impl Default for CommandQueue {
    fn default() -> Self {
        CommandQueue::new()
    }
}
impl CommandQueue {
    pub fn new() -> Self {
        Self {
            list: Vec::<Command>::new(),
        }
    }
    pub fn add(&mut self, CommandQueue: Command) {
        self.list.push(CommandQueue);
    }
    pub fn get(&mut self) -> Option<Command> {
        self.list.pop()
    }
}
