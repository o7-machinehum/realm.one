use log::info;
use crate::{
    components::{Orientation},
};

#[derive(Debug)]
pub enum Inputs {
    Move(Orientation),
    Melee,
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
