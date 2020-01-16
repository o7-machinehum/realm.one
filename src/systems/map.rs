use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, Write, Entities, System, SystemData, World, WriteStorage, Entity};
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
        Write<'s, Room>,
        Read<'s, SpritesContainer>,
        Entities<'s>,
    );
    
    /// Should ONLY be called in a re-draw event of the map
    /// Resource room should be updated with the newest room
    fn run(&mut self, (mut transforms, mut sprite_renders, mut tiles_pos, mut room, mut container, entities): Self::SystemData) {
        if room.update {
            // Delete old tiles
            for entity in room.tile_ent.iter() {
                entities.delete(*entity);
            }
            
            // Add new tiles
            let mut entList: Vec<Entity> = Vec::new();
            for (z, layer) in room.map.layers.iter().enumerate() {
                for (x, row) in layer.tiles.iter().rev().enumerate() {
                    for (y, col) in row.iter().enumerate() {
                        if col.gid != 0 {
                            let mut loc = TilePosition::new(x, y, z, col.gid as usize - 1);
                            let mut transform = loc.to_trans(); 
                            entList.push(
                                entities.build_entity()
                                    .with(container.sprites[loc.gid].clone(), &mut sprite_renders) 
                                    .with(transform, &mut transforms)
                                    .with(loc, &mut tiles_pos) 
                                    .build()
                            );
                        }
                    }
                }
            }
            room.update = false;
            room.tile_ent.append(&mut entList); 
        }
    }
}
