use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, Write, Entities, System, SystemData, World, WriteStorage, Entity};
use amethyst::renderer::SpriteRender;

use crate::map::{Room, TilePosition, SpritesContainer};
use log::info;
use crate::network::{IO, Cmd};

#[derive(SystemDesc)]
pub struct MapSystem ;

impl<'s> System<'s> for MapSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, TilePosition>,
        Write<'s, Room>,
        Write<'s, IO>,
        Read<'s, SpritesContainer>,
        Entities<'s>,
    );
    
    /// Should ONLY be called in a re-draw event of the map
    /// Resource room should be updated with the newest room
    fn run(&mut self, (mut transforms, mut sprite_renders, mut tiles_pos, mut room, mut io, container, entities): Self::SystemData) {
        for element in io.I.pop() {
            match &element.cmd {
                Cmd::TransferMap(name, data) => room.change(name.to_string(), data.to_string()),
                _ => io.I.push(element), 
            }
        }
        
        if room.update {
            // Delete old tiles
            for entity in room.tile_ent.iter() {
                entities.delete(*entity).expect("Failed to delete old map entities");
            }
            
            // Add new tiles
            let mut ent_list: Vec<Entity> = Vec::new();
            for (z, layer) in room.map.layers.iter().enumerate() {
                for (x, row) in layer.tiles.iter().rev().enumerate() {
                    for (y, col) in row.iter().enumerate() {
                        if col.gid != 0 {
                            let mut loc = TilePosition::new(x, y, z, col.gid as usize - 1);
                            let transform = loc.to_trans(); 
                            ent_list.push(
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
            room.tile_ent.append(&mut ent_list); 
        }
    }
}
