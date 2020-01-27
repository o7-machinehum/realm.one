use amethyst::{
    renderer::SpriteRender,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};
use std::time::Instant;
use serde::{Serialize, Deserialize};
use crate::constants;
use std::net::{SocketAddr};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Orientation {
    South,
    West,
    East,
    North,
}

pub struct PlayerList {
    pub list: Vec<PlayerComponent>,
}

impl Default for PlayerList {
    fn default() -> Self {
        Self{ list: Vec::new(), } 
    }
}

/// Client Side player component
#[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerComponent {
    pub id: u32,       // Change this to idNum
    pub ip: SocketAddr, 
    pub modified: bool, 
    pub name: String,
    pub room: String,
    pub x: f32,          
    pub y: f32, 
    pub north: usize,      
    pub east: usize, 
    pub south: usize,
    pub west: usize, 
    pub orientation: Orientation,
}

impl PlayerComponent {
    pub fn new(p: PlayerComponent) -> Self {
        p 
    }
    
    pub fn update_orientation(&mut self, x: &f32, y: &f32) {
        let x = *x;
        let y = *y;

        if x > 0. {
            self.orientation = Orientation::East;
        } else if x < 0. {
            self.orientation = Orientation::West;
        } else if y > 0. {
            self.orientation = Orientation::North;
        } else if y < 0. {
            self.orientation = Orientation::South;
        }
    }
    
    pub fn walk(&mut self, x: &f32, y: &f32) {
        self.x += *x * constants::PLAYER_MOVE;
        self.y += *y * constants::PLAYER_MOVE;
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        1.0 
    }

    pub fn trans(&self) -> Transform {
        let mut tr = Transform::default();
        tr.set_translation_xyz(self.x(), self.y(), self.z()); 
        tr
    }
    
    pub fn get_orientated(&self, sprites: &Vec<SpriteRender>) -> SpriteRender {
        match self.orientation {
            Orientation::North=> return sprites[self.north].clone(),
            Orientation::South=> return sprites[self.south].clone(),
            Orientation::East => return sprites[self.east].clone(),
            Orientation::West => return sprites[self.west].clone(),
        }
    }
}

impl Component for PlayerComponent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
