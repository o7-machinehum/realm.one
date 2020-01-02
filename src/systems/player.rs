use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::InputHandler;

use std::time::Instant;

use crate::components::PlayerComponent;
use crate::key_bindings::{MovementBindingTypes, AxisBinding, ActionBinding};
use log::info;

const TILE_SIZE : f32 = 16.0;
const MOVEMENT_DELAY_MS : u128 = 150;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PlayerComponent>,
        Read<'s, InputHandler<MovementBindingTypes>>,
    );

    fn run(&mut self, (mut transforms, mut players, input): Self::SystemData) {
        for (player, transform) in (&mut players, &mut transforms).join() {  
            let now = Instant::now();

            if now.duration_since(player.last_movement_instant).as_millis() >= MOVEMENT_DELAY_MS {
                let horizontal = input
                    .axis_value(&AxisBinding::Horizontal)
                    .unwrap_or(0.0);
                let vertical = input
                    .axis_value(&AxisBinding::Vertical)
                    .unwrap_or(0.0);
                
                player.last_movement_instant = now.clone();

                transform.move_up(vertical * TILE_SIZE);
                transform.move_right(horizontal * TILE_SIZE);
            }
        } 
    }
}
