use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
    core::transform::Transform,
};

use serde::{Serialize, Deserialize};
use crate::components::{PlayerComponent};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MeleeAnimation {
    end_stance: usize,
    at_stance: usize,
    pub sword: usize,
    pub sword_pos: Transform,
    swing_time: f32,
    elapsed_time: f32,
    pec: f32,
    pub draw_sword: bool,
}

impl MeleeAnimation {
    pub fn new(pl: &PlayerComponent) -> Self { 
        Self {
            end_stance: pl.get_dir(),
            at_stance: pl.get_at(),
            sword: pl.get_sword(),
            sword_pos: pl.get_sword_pos(),
            swing_time: 0.25,
            elapsed_time: 0.0,
            pec: 0.0,
            draw_sword: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed_time += dt;
        self.pec = self.elapsed_time / self.swing_time;
    }
    
    /// Get the current animation sequence
    pub fn get_seq(&mut self) -> Option<usize> {
        //TODO: RATE LIMIT THIS (DON'T ALWAYS RETURN SOME)
        if self.pec > 0.9 {
            return Some(self.end_stance);
        }
        else {
            return Some(self.at_stance);
        }
        None
    }

    pub fn delete(&self) -> bool {
        if self.pec >= 1.0 {
            return true
        }
        false
    }
}

impl Component for MeleeAnimation {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
