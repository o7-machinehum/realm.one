use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

use log::info;
use serde::{Serialize, Deserialize};
use nalgebra::base::{Vector3, Matrix4};
use nalgebra::geometry::Isometry3;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Move {
    point_a: Vector3<f32>,
    point_b: Vector3<f32>,
    vel: f32,
    time: f32,
    elapsed_time: f32,
    pec: f32,
}

impl Move {
    pub fn new(point_a: Vector3<f32>, point_b: Vector3<f32>, time: f32)
        -> Self 
    {
        Self {
            point_a,
            point_b,
            vel: 1.0,  // TODO: This should be calculated
            time, 
            elapsed_time: 0.0,
            pec: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.elapsed_time += dt;
    }

    pub fn delete(&self) -> bool {
        if self.pec >= 1.0 {
            return true
        }
        false
    }
    
    pub fn end(&self) -> Vector3<f32>{
        self.point_b.clone()
    }

    pub fn pos(&mut self) -> Vector3<f32> {
        // This should run from 0 -> 1
        self.pec = self.elapsed_time / self.time;
        lerp(&self.point_a, &self.point_b, self.pec) 
    }
}

impl Component for Move {
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
