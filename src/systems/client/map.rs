use amethyst::{
    core::{SystemDesc, bundle::SystemBundle, Transform},
    derive::SystemDesc,
    ecs::{Write, World, Read, System, SystemData, DispatcherBuilder, WriteStorage, Entity, Entities},
    renderer::SpriteRender,
    shrev::{EventChannel, ReaderId},
    Result, 
};

use crate::{
    map::{Room, TilePosition, Layers},
    resources::{SpritesContainer},
};

/// Events that pertain to the Auth System
#[derive(Debug)]
pub enum MapEvent {
    TransferMap( String ), 
}

#[derive(SystemDesc)]
pub struct MapSystem {
    event_reader: ReaderId<MapEvent>,
}


pub struct MapSystemBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for MapSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            MapSystemDesc::default().build(world),
            "map_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct MapSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, MapSystem> for MapSystemDesc {
    fn build(self, world: &mut World) -> MapSystem {
        <MapSystem as System<'_>>::SystemData::setup(world);
        let event_reader = world
            .fetch_mut::<EventChannel<MapEvent>>()
            .register_reader();
        MapSystem{ event_reader }
    }
}

impl<'s> System<'s> for MapSystem{
    type SystemData = (
        Read <'s, EventChannel<MapEvent>>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, TilePosition>,
        Write<'s, Room>,
        Read<'s, SpritesContainer>,
        Entities<'s>,
    );
    
    /// Should ONLY be called in a re-draw event of the map
    /// Resource room should be updated with the newest room
    fn run(&mut self, (ev, mut transforms, mut sprite_renders, mut tiles_pos, mut room, container, entities): Self::SystemData) {
        for event in ev.read(&mut self.event_reader) {
            match event{
                MapEvent::TransferMap(name) => room.change(name.to_string()),
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
                if z == Layers::Monsters as usize { break; }; // Monster Layer, don't draw.
                for (x, row) in layer.tiles.iter().rev().enumerate() {
                    for (y, col) in row.iter().enumerate() {
                        if col.gid != 0 {
                            let mut loc = TilePosition::new(y, x, z, col.gid as usize - 1);
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
