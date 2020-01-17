use amethyst::{
    renderer::SpriteRender,
    core::transform::Transform,
    prelude::*,
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};
use std::time::Instant;

#[derive(Clone)]
pub enum Orientation {
    South,
    West,
    East,
    North,
}

/// Server Size player components
// pub struct ServerPlayerComponent {
//     x: f32,
//     y: f32,
//     no: usize, 
//     ea: usize, 
//     so: usize,
//     we: usize, 
// }

/// Client Side player component
pub struct PlayerComponent {
    pub x: f32,
    pub y: f32,
    pub orientation: Orientation,
    pub n: SpriteRender,
    pub e: SpriteRender,
    pub s: SpriteRender,
    pub w: SpriteRender,
    pub last_movement_instant: Instant,
}

impl PlayerComponent {
    pub fn new( x: f32, y: f32, (no, ea, so, we) : (usize, usize, usize, usize) , sprites: &Vec<SpriteRender>) -> Self {
        Self {
            x,
            y,
            n: sprites[no].clone(), 
            e: sprites[ea].clone(), 
            s: sprites[so].clone(), 
            w: sprites[we].clone(),
            orientation: Orientation::South,
            last_movement_instant: Instant::now(),
        }
    }
    
    pub fn insert(self, world: &mut World) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(self.x, self.y, 1.0); 

        // Create a player entity.
        world
            .create_entity()
            .with(self.n.clone()) 
            .with(self)
            .with(transform)
            .build();
    }

    pub fn get_orientated(&self) -> SpriteRender{
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
