use amethyst::{
    renderer::SpriteRender,
    core::transform::Transform,
    prelude::*,
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};
use std::time::Instant;
use serde::{Serialize, Deserialize};
use crate::constants;
use log::info;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Orientation {
    South,
    West,
    East,
    North,
}

pub struct PlayerList {
    pub list: Vec<PlayerInfo>,
}

impl Default for PlayerList {
    fn default() -> Self {
        Self{ list: Vec::new(), } 
    }
}

/// Server Size player components
#[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerInfo {
    pub id: u32,       // Change this to idNum
    pub modified: bool, 
    pub name: String,
    pub room: String,
    pub x: f32,          
    pub y: f32, 
    pub no: usize,      
    pub ea: usize, 
    pub so: usize,
    pub we: usize, 
}

/// Client Side player component
pub struct PlayerComponent {
    pub orientation: Orientation,
    n: SpriteRender,
    e: SpriteRender,
    s: SpriteRender,
    w: SpriteRender,
    pub last_movement_instant: Instant,
    p: PlayerInfo,
}

impl PlayerComponent {
    pub fn new(p: PlayerInfo, sprites: &Vec<SpriteRender>) -> Self {
        
        Self {
            n: sprites[p.no].clone(), 
            e: sprites[p.ea].clone(), 
            s: sprites[p.so].clone(), 
            w: sprites[p.we].clone(),
            orientation: Orientation::South,
            last_movement_instant: Instant::now(),
            p,
        }
    }
    
    // pub fn action(&self, action: PlayerAction) {
    //     match action {
    //         Move => (),
    //         _    => (),
    //     }
    // }

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
        self.p.x += *x * constants::PLAYER_MOVE;
        self.p.y += *y * constants::PLAYER_MOVE;
    }

    pub fn x(&self) -> f32 {
        self.p.x
    }

    pub fn y(&self) -> f32 {
        self.p.y
    }

    pub fn z(&self) -> f32 {
        1.0 
    }

    pub fn trans(&self) -> Transform {
        let mut tr = Transform::default();
        tr.set_translation_xyz(self.x(), self.y(), self.z()); 
        tr
    }
    
    pub fn get_orientated(&self) -> SpriteRender {
        match self.orientation {
            Orientation::North=> return self.n.clone(),
            Orientation::South=> return self.s.clone(),
            Orientation::East => return self.e.clone(),
            Orientation::West => return self.w.clone(),
        }
    }
}

impl Component for PlayerComponent {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
