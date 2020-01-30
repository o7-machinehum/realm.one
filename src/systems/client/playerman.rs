use amethyst::core::{Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, Write, System, SystemData, WriteStorage, Join};

use amethyst::renderer::SpriteRender;


use log::info;

use crate::components::{PlayerComponent};

use crate::map::{SpritesContainer};
use crate::network::{IO, Cmd};


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
 
    fn run(&mut self, (mut transforms, mut players, mut sprite_renders, mut io, _s): Self::SystemData) {
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
                Cmd::RemovePlayer(_ip) => {
                    // Remove the player and delete the
                    // components and entities
                }, 
                _ => io.i.push(element), 
            }
        }
    }
}
