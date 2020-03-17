use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

use log::info;
use serde::{Serialize, Deserialize};
use nalgebra::base::{Vector3, Matrix4};
use nalgebra::geometry::Isometry3;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Step {
    RightStep = -1,
    NoStep = 0,
    LeftStep = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WalkAnimation {
    current_frame: usize,
    start: Vector3<f32>,
    end: Vector3<f32>,
    walk_time: f32,
    elapsed_time: f32,
    footing: Step,
    pec: f32,
}

impl WalkAnimation {
    pub fn new(walk_time: f32, start: Vector3<f32>, end: Vector3<f32>)
        -> WalkAnimation
    {
        WalkAnimation {
            current_frame: 0,
            start,
            end,
            walk_time,
            elapsed_time: 0.0,
            footing: Step::NoStep, 
            pec: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed_time += dt;
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
        if self.pec >= 1.0 {
            return true
        }
        false
    }

    pub fn pos(&mut self) -> Vector3<f32> {
        // This should run from 0 -> 1
        self.pec = self.elapsed_time / self.walk_time;
        lerp(&self.start, &self.end, self.pec) 
    }
}

impl Component for WalkAnimation {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

// just take the transform.isometry() to get an isometry and then param should be a value between 0 and 1
// at 0, you'll get the value of the first param
// at 1, you'll get the value of the second
// at 0.5, it'll be halfway inbetween
// don't ask me what'll happen to rotations
fn lerp(zero: &Vector3<f32>, one: &Vector3<f32>, param: f32) -> Vector3<f32> {
    zero * (1.0 - param) + (param) * one
}
