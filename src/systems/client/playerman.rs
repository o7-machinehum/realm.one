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
                    for (transform, player, sprite_render) in (&mut transforms, &mut players, &mut sprite_renders).join() { 
                        if player.name == new.name {
                            info!("Updating Player: {:?}", player);

                            if *transform.translation() != new.xyz() { 
                                transform.set_translation(new.xyz()); 
                            }

                            if player.orientation != new.orientation {
                                sprite_render.sprite_number = new.get_dir();
                            }
                            
                            *player = new.clone();
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
