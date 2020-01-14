use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, Entities, System, SystemData, World, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::network;

use std::time::Instant;

use crate::components::{PlayerComponent, Orientation};
use crate::key_bindings::{MovementBindingTypes, AxisBinding};
use crate::map::{Room, Adj};
use crate::events::{Events};

use crate::constants;

#[derive(SystemDesc)]
pub struct PlayerSystem ;
// {
//     reader: Option<ReaderId<Events>>,
// }

// impl PlayerSystem {
//     pub fn new(reader: ReaderId<Events>) -> Self {
//         PlayerSystem{ reader }
//     }
// }

impl<'s> System<'s> for PlayerSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PlayerComponent>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Room>,
        Read<'s, EventChannel<Events>>,
        Entities<'s>,
        Read<'s, InputHandler<MovementBindingTypes>>,
    );

    fn run(&mut self, (mut transforms, mut players, mut sprite_renders, mut rooms, mut events, entities, input): Self::SystemData) {
        // for event in events.read(reader) {
        //     println!("Received an event: {:?}", event);
        // }
        
        for (entity, player, transform) in (&*entities, &mut players, &mut transforms).join() {  
            // let reader = readers
            //     .entry(e)
            //     .expect("Cannot get reader")
            //     .or_insert_with(|| network::Reader(connection.register_reader()));
            
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

                sprite_renders.insert(entity, player.get_orientated());

                for room in (&mut rooms).join() {
                    let adj: Adj = room.get_adj(transform);
                    if room.allowed_move(transform, horizontal, vertical, adj){
                        transform.move_up(vertical * constants::PLAYER_MOVE );
                        transform.move_right(horizontal * constants::PLAYER_MOVE );
                    }
                }
            }
        } 
    }
}
