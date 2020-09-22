use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Step {
    RightStep = -1,
    NoStep = 0,
    LeftStep = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WalkAnimation {
    current_frame: usize,
    walk_time: f32,
    elapsed_time: f32,
    footing: Step,
    pec: f32,
}

impl WalkAnimation {
    pub fn new(walk_time: f32)
        -> WalkAnimation
    {
        WalkAnimation {
            walk_time,
            current_frame: 0,
            elapsed_time: 0.0,
            footing: Step::NoStep, 
            pec: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed_time += dt;
        self.pec = self.elapsed_time / self.walk_time;
    }

    pub fn get_seq(&mut self) -> Option<i32> {
        if self.footing == Step::NoStep && self.pec < 0.33 {
            self.footing = Step::RightStep;
            return Some(self.footing.clone() as i32);
        }
        if self.footing == Step::RightStep && self.pec > 0.33 && self.pec < 0.66 {
            self.footing = Step::LeftStep;
            return Some(self.footing.clone()as i32);
        }
        if self.footing == Step::RightStep && self.pec > 0.66 {
            self.footing = Step::NoStep;
            return Some(self.footing.clone() as i32);
        }
        None
    }

    pub fn delete(&self) -> bool {
        self.pec >= 1.0
    }
}

impl Component for WalkAnimation {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
