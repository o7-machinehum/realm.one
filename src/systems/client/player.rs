use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, Write, Entities, System, SystemData, World, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;

use std::time::Instant;
use log::info;

use crate::components::{PlayerComponent, Orientation, PlayerList};
use crate::key_bindings::{MovementBindingTypes, AxisBinding};
use crate::map::{Room, Adj, SpritesContainer};

use crate::constants;

#[derive(SystemDesc)]
pub struct PlayerSystem ;

impl<'s> System<'s> for PlayerSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PlayerComponent>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, Room>,
        Entities<'s>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Write<'s, PlayerList>,
        Read<'s, SpritesContainer>,
    );

    fn run(&mut self, (mut transforms, mut players, mut sprite_renders, room, entities, input, mut p, s): Self::SystemData) {
        match p.list.pop() {
            Some(pl) => {
                info!("Inserting Player"); 
                let player = PlayerComponent::new(pl, &s.sprites);
                entities
                    .build_entity()
                    .with(player.trans.clone(), &mut transforms)
                    .with(player.get_orientated().clone(), &mut sprite_renders)
                    .with(player, &mut players) 
                    .build();
            }
            None => {},
        }
        
        for (entity, player, transform) in (&*entities, &mut players, &mut transforms).join() {  
            let now = Instant::now();

            if now.duration_since(player.last_movement_instant).as_millis() >= constants::MOVEMENT_DELAY_MS {
                let horizontal = input
                    .axis_value(&AxisBinding::Horizontal)
                    .unwrap_or(0.0);
                let vertical = input
                    .axis_value(&AxisBinding::Vertical)
                    .unwrap_or(0.0);
                
                if horizontal == 0. && vertical == 0. {
                    return;
                }
                
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
                sprite_renders.insert(entity, player.get_orientated()).expect("Failed to insert orientated player!");

                let adj: Adj = room.get_adj(transform);
                if room.allowed_move(transform, horizontal, vertical, adj){
                    transform.move_up(vertical * constants::PLAYER_MOVE );
                    transform.move_right(horizontal * constants::PLAYER_MOVE );
                }
            }
        } 
    }
}
