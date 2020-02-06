use amethyst::{
    renderer::SpriteRender,
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

use serde::{Serialize, Deserialize};
use crate::{constants};
use std::net::{SocketAddr};
use nalgebra::base::Vector3;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Orientation {
    South,
    West,
    East,
    North,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Skins {
    Nude,
    Male,
    Female,
    Skeleton,
    Slime,
    Bat,
    Ghost,
    Spider,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Outfit {
    pub n: usize,      
    pub e: usize, 
    pub s: usize,
    pub w: usize, 
}

pub fn get_outfit(skin: &Skins) -> Outfit {
    match skin {
        Skins::Nude     => Outfit { n: 312, e: 300, s: 276, w: 288 },
        Skins::Male     => Outfit { n: 315, e: 303, s: 279, w: 291 },
        Skins::Female   => Outfit { n: 318, e: 306, s: 282, w: 294 },
        Skins::Skeleton => Outfit { n: 321, e: 309, s: 285, w: 297 },
        Skins::Slime    => Outfit { n: 360, e: 349, s: 324, w: 336 },
        Skins::Bat      => Outfit { n: 363, e: 352, s: 327, w: 339 },
        Skins::Ghost    => Outfit { n: 366, e: 355, s: 330, w: 342},
        Skins::Spider   => Outfit { n: 369, e: 358, s: 233, w: 245 },
    }
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
