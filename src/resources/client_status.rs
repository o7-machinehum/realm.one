use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};

pub struct ClientStatus{
    pub connected: bool,
}

impl ClientStatus {
    pub fn new() -> Self{
        Self {
            connected: false,
        }
    }
}

impl Default for ClientStatus{
    fn default() -> Self {
        Self {
            connected: false,
        } 
    }
}
