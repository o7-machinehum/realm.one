use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, Write, System, SystemData, WriteStorage, Join, Entities};
use amethyst::renderer::SpriteRender;
use log::info;

use crate::{ 
    components::{PlayerComponent},
    network::Cmd,
    resources::{IO, SpritesContainer},
};

#[derive(SystemDesc)]
pub struct PlayerManSystem; 

impl<'s> System<'s> for PlayerManSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PlayerComponent>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, IO>,
        Read<'s, SpritesContainer>,
        Entities<'s>,
    );
 
    fn run(&mut self, (mut transforms, mut players, mut sprite_renders, mut io, _s, entities): Self::SystemData) {
        for element in io.i.pop() {
            match element.cmd {
                Cmd::UpdatePlayer(new) => {
                    for (transform, player, _sprite_render) in (&mut transforms, &mut players, &mut sprite_renders).join() { 
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
                Cmd::RemovePlayer(ip) => {
                    info!("Removing Player of ip: {}", ip);
                    for (e, player) in (&*entities, &mut players).join() { 
                        if player.ip == ip {
                            entities.delete(e).expect("Failed to delete old player entities");
                        }
                    } 
                }, 
                _ => io.i.push(element), 
            }
        }
    }
}
