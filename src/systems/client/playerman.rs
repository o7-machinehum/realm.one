use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, Write, Entities, Entity, System, SystemData, World, WriteStorage, Join};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;

use std::time::Instant;
use log::info;

use crate::components::{PlayerComponent, Action};
use crate::key_bindings::{MovementBindingTypes, AxisBinding};
use crate::map::{Room, Adj, SpritesContainer};
use crate::network::{Pack, IO, Cmd};
use crate::constants;

#[derive(SystemDesc)]
pub struct PlayerManSystem; 

impl<'s> System<'s> for PlayerManSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PlayerComponent>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, IO>,
        Read<'s, SpritesContainer>,
    );
 
    fn run(&mut self, (mut transforms, mut players, mut sprite_renders, mut io, s): Self::SystemData) {
        for element in io.i.pop() {
            match element.cmd {
                Cmd::UpdatePlayer(new) => {
                    for (mut transform, mut player, mut sprite_render) in (&mut transforms, &mut players, &mut sprite_renders).join() { 
                        if player.name == new.name {
                            info!("Updating Player: {:?}", player);
                            *player = new.clone();

                            info!("{:?}", *transform.translation());
                            info!("{:?}", new.xyz());

                            if *transform.translation() != new.xyz() { 
                                info!("Replacing Translation");
                                transform.set_translation(player.xyz()); 
                            }
                        }
                    }        
                }, 
                _ => io.i.push(element), 
            }
        }
    }
}
