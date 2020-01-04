use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, Entities, System, SystemData, World, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;

use std::time::Instant;

use crate::components::PlayerComponent;
use crate::character_sprites::{Orientation, get_oriented_sprite};
use crate::key_bindings::{MovementBindingTypes, AxisBinding};
use crate::map::Room;

const TILE_SIZE : f32 = 16.0;
const MOVEMENT_DELAY_MS : u128 = 150;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PlayerComponent>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Room>,
        Entities<'s>,
        Read<'s, InputHandler<MovementBindingTypes>>,
    );

    fn run(&mut self, (mut transforms, mut players, mut sprite_renders, mut rooms, entities, input): Self::SystemData) {
        for (entity, player, transform) in (&*entities, &mut players, &mut transforms).join() {  
            let now = Instant::now();

            if now.duration_since(player.last_movement_instant).as_millis() >= MOVEMENT_DELAY_MS {
                let horizontal = input
                    .axis_value(&AxisBinding::Horizontal)
                    .unwrap_or(0.0);
                let vertical = input
                    .axis_value(&AxisBinding::Vertical)
                    .unwrap_or(0.0);

                let orientation : Orientation;
                if horizontal > 0. {
                    orientation = Orientation::East;
                } else if horizontal < 0. {
                    orientation = Orientation::West;
                } else if vertical > 0. {
                    orientation = Orientation::North;
                } else if vertical < 0. {
                    orientation = Orientation::South;
                } else {
                    orientation = player.orientation.clone()
                }
                player.orientation = orientation.clone();

                player.last_movement_instant = now.clone();
                
                sprite_renders.insert(entity, get_oriented_sprite(player.spritesheet_handle.clone(), orientation));
                
                for (room) in (&mut rooms).join() {
                    let mut adj_tiles: Vec<tiled::Properties> = room.get_adj(transform);

                    transform.move_up(vertical * TILE_SIZE);
                    transform.move_right(horizontal * TILE_SIZE);
                } 
            }
        } 
    }
}
