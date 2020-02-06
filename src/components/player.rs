use amethyst::{
    renderer::SpriteRender,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, FlaggedStorage, WriteStorage, Join},
};

use serde::{Serialize, Deserialize};
use crate::{constants};
use crate::components::{Skins, Outfit};
use std::net::{SocketAddr};
use nalgebra::base::Vector3;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Orientation {
    South,
    West,
    East,
    North,
}

/// Client Side player component
#[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PlayerComponent {
    pub name: String,
    pub ip: SocketAddr, 
    pub room: String,
    pub x: f32,          
    pub y: f32, 
    pub skin: Outfit,
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
    
    pub fn walk(&mut self) {
        match self.orientation {
            Orientation::North => self.y += constants::PLAYER_MOVE,
            Orientation::South => self.y -= constants::PLAYER_MOVE,
            Orientation::East  => self.x += constants::PLAYER_MOVE,
            Orientation::West  => self.x -= constants::PLAYER_MOVE,
        }
    }

    pub fn in_front(&self) -> Transform {
        let mut tr = self.trans();
        
        match self.orientation {
            Orientation::North => tr.move_up(constants::PLAYER_MOVE),  
            Orientation::South => tr.move_down(constants::PLAYER_MOVE),
            Orientation::East  => tr.move_right(constants::PLAYER_MOVE),
            Orientation::West  => tr.move_left(constants::PLAYER_MOVE),
        };
        tr
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

    pub fn xyz(&self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z()) 
    }

    pub fn trans(&self) -> Transform {
        let mut tr = Transform::default();
        tr.set_translation_xyz(self.x(), self.y(), self.z()); 
        tr
    }
    
    pub fn get_orientated(&self, sprites: &Vec<SpriteRender>) -> SpriteRender {
        match self.orientation {
            Orientation::North=> return sprites[self.skin.n].clone(),
            Orientation::South=> return sprites[self.skin.s].clone(),
            Orientation::East => return sprites[self.skin.e].clone(),
            Orientation::West => return sprites[self.skin.w].clone(),
        }
    }

    pub fn get_dir(&self) -> usize{
        match self.orientation {
            Orientation::North => self.skin.n,
            Orientation::South => self.skin.s,
            Orientation::East  => self.skin.e,
            Orientation::West  => self.skin.w,
        }
    }
}

impl Component for PlayerComponent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
