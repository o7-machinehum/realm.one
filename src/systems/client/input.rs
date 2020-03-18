use amethyst::{
    derive::SystemDesc,
    ecs::{Read, Write, System, SystemData},
    input::InputHandler,
    renderer::resources::Tint,
};

use std::time::Instant;
use log::info;

use crate::{
    components::{Orientation},
    key_bindings::{MovementBindingTypes, AxisBinding, ActionBinding},
    resources::{Inputs, Input},
};

#[derive(SystemDesc)]
pub struct InputSystem {
    mv_latch: bool,
    melee_latch: bool,
} 

impl InputSystem {
    pub fn new() -> Self {
        Self {
            mv_latch: false,
            melee_latch: false,
        }
    }
}

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        Read<'s, InputHandler<MovementBindingTypes>>,
        Write<'s, Input>,
    );
 
    fn run(&mut self, ( input, mut input_res): Self::SystemData) {
        match input.axis_value(&AxisBinding::Horizontal) {
            Some(value) => {
                if value > 0.0 && !self.mv_latch {
                    input_res.add(Inputs::Move(Orientation::East));
                    self.mv_latch = true;
                }

                if value < 0.0 && !self.mv_latch {
                    input_res.add(Inputs::Move(Orientation::West));
                    self.mv_latch = true;
                }
                
                if value == 0.0 {
                    self.mv_latch = false;
                }

            },
            None => (),
        }
        
        match input.axis_value(&AxisBinding::Vertical) {
            Some(value) => {
                if value > 0.0 && !self.mv_latch {
                    input_res.add(Inputs::Move(Orientation::North));
                    self.mv_latch = true;
                }

                if value < 0.0 && !self.mv_latch {
                    input_res.add(Inputs::Move(Orientation::South));
                    self.mv_latch = true;
                }
                
                if value == 0.0 {
                    self.mv_latch = false;
                }

            },
            None => (),
        }

        match input.action_is_down(&ActionBinding::Melee) {
            Some(value) => {
                if value == true {
                    input_res.add(Inputs::Melee);
                }
            },
            None => (),
        }
    }
}
