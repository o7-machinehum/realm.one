use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, Entities, System, SystemData, World, WriteStorage};
use amethyst::renderer::SpriteRender;

use crate::map::{Room, TilePosition, SpritesContainer};
use log::info;

#[derive(SystemDesc)]
pub struct MapSystem ;

impl<'s> System<'s> for MapSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, TilePosition>,
        Read<'s, Room>,
        Read<'s, SpritesContainer>,
        Entities<'s>,
    );
    
    /// Should ONLY be called in a re-draw event of the map
    /// Resource room should be updated with the newest room
    fn run(&mut self, (mut transforms, mut sprite_renders, mut tiles_pos, mut room, mut container, entities): Self::SystemData) {
        for (sprite_render, transform, pos) in (&mut sprite_renders, &mut transforms, &mut tiles_pos).join() { 
            match room.diff_gid(pos) {
                Some(gid) => {
                    pos.gid = gid;                                           // Change the gid in the position 
                    if gid == 0 {
                        //TODO: This is just inserting a blank sprites into the spriterender
                        // I don't know if this is the best way to do this.
                        *sprite_render = container.sprites[49].clone();      
                    }
                    else {
                        *sprite_render = container.sprites[gid - 1].clone(); // Change the sprite
                    }
                },
                None => {},
            }
        }
    }
}
