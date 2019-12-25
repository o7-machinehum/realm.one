use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::InputHandler;

use crate::state::{Player};
use crate::key_bindings::{MovementBindingTypes, AxisBinding, ActionBinding};
use log::info;

const TILE_SIZE : f32 = 16.0;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<MovementBindingTypes>>,
    );

    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {  
            let horizontal = input
                .axis_value(&AxisBinding::Horizontal)
                .unwrap_or(0.0);
            let vertical = input
                .axis_value(&AxisBinding::Vertical)
                .unwrap_or(0.0);

            transform.move_up(vertical * TILE_SIZE);
            transform.move_right(horizontal * TILE_SIZE);
        } 
    }
}
