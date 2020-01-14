use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ecs::{Entities, Component, DenseVecStorage, FlaggedStorage},
};

extern crate tiled;
use std::{
    fs::File,
    io::BufReader,
    path::Path,
};

use log::info;
use crate::constants;
use crate::mech::colision;

enum Layers {
    L1 = 0,
    L2,
    L3,
    L4,
    L5,
    L6,
}

pub struct Room {
    pub current: tiled::Map,   // Current room 
    pub len_width: Vec<i32>,   // How many tiles in the png
    pub sprites: Vec<SpriteRender>,
}

pub struct Adj {
    pub cur: Option<tiled::Properties>,
    pub n: Option<tiled::Properties>,
    pub e: Option<tiled::Properties>,
    pub s: Option<tiled::Properties>,
    pub w: Option<tiled::Properties>,
}

impl Component for Room{
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

// Comment
impl Room {
    pub fn new(file_name: String) -> Self {
        let file = File::open(&Path::new(&file_name)).unwrap();
    	let reader = BufReader::new(file);
        // let map =  tiled::parse(reader).unwrap();
        
        let map =  tiled::parse_with_path(reader, &Path::new("resources/sprites/basictiles.tsx")).unwrap();

        // info!("{:?}", map.layers[0].tiles);
        // info!("Width/Height: {}, {}, ", map.width, map.height);

        Self {
            len_width: Room::count_tiles(&map), 
            current: map,
            sprites: Vec::new(), 
        }
    }

    pub fn change(&mut self, newMap: tiled::Map) {
        self.len_width = Room::count_tiles(&newMap); 
        self.current = newMap;
    }

    fn count_tiles(map: &tiled::Map) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        for sets in &map.tilesets {
            v.push((sets.images[0].width / sets.tile_width as i32) * (sets.images[0].height / sets.tile_height as i32)) 
        }

        info!("Tiles in the images: {:?}", v);

        v
    }

    fn draw_layer(&mut self, world: &mut World, layer: Layers) {
        let mut x;
        let mut y = 0.0;
        for row in self.current.layers[layer as usize].tiles.iter().rev() {
            x = 0.0;
            y += constants::TILE_SIZE;

            for col in row.iter() {
                x += constants::TILE_SIZE; 

                let mut transform = Transform::default();
                transform.set_translation_xyz(x, y, 0.);
                
                if col.gid != 0 {
                    world
                        .create_entity()
                        .with(self.sprites[col.gid as usize - 1].clone())
                        .with(transform)
                        .build();
                }
            }
        }
    }
    
    pub fn draw_room(&mut self, world: &mut World) {
        // self.draw_layer(world, Layers::L6);
        self.draw_layer(world, Layers::L5);
        self.draw_layer(world, Layers::L4);
        self.draw_layer(world, Layers::L3);
        self.draw_layer(world, Layers::L2);
        self.draw_layer(world, Layers::L1);
    }
    
    fn draw_layer_ent(&mut self, ent: Entities, layer: Layers) {
        let mut x;
        let mut y = 0.0;
        for row in self.current.layers[layer as usize].tiles.iter().rev() {
            x = 0.0;
            y += constants::TILE_SIZE;

            for col in row.iter() {
                x += constants::TILE_SIZE; 

                let mut transform = Transform::default();
                transform.set_translation_xyz(x, y, 0.);
                
                if col.gid != 0 {
                    
                ent
                    .build_entity()
                    .with(self.sprites[col.gid as usize - 1].clone())
                    .with(transform)
                    .build();
                }
            }
        }
    }
     
    pub fn draw_room_ent(&mut self, ent: Entities) {
        self.draw_layer(ent, Layers::L6);
        // self.draw_layer(world, Layers::L5);
        // self.draw_layer(world, Layers::L4);
        // self.draw_layer(world, Layers::L3);
        // self.draw_layer(world, Layers::L2);
        // self.draw_layer(world, Layers::L1);
    }

    pub fn load_sprites(&mut self, world: &mut World) {
        let mut ii = 0;

        for sets in &self.current.tilesets {
            // Load the texture for our sprites. We'll later need to
            // add a handle to this texture to our `SpriteRender`s, so
            // we need to keep a reference to it.
            let texture_handle = {
                let loader = world.read_resource::<Loader>();
                let texture_storage = world.read_resource::<AssetStorage<Texture>>();

                loader.load(
                    // "sprites/basictiles.png",
                    format!("sprites/{}.png", sets.name), 
                    ImageFormat::default(),
                    (),
                    &texture_storage,
                )
            };

            // Load the spritesheet definition file, which contains metadata on our
            // spritesheet texture.
            let sheet_handle = {
                let loader = world.read_resource::<Loader>();
                let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

                loader.load(
                    // "sprites/basictiles.ron",
                    format!("sprites/{}.ron", sets.name), 
                    SpriteSheetFormat(texture_handle),
                    (),
                    &sheet_storage,
                )
            };
 
            for i in 0..self.len_width[ii] { 
                self.sprites.push(SpriteRender {
                    sprite_sheet: sheet_handle.clone(),
                    sprite_number: i as usize,
                });
            };
            ii += 1;
        };
    }

    // Convert world coordinates to tiled coordinates
    fn world_2_tiled(&mut self, (x, y): (i32, i32)) -> (i32, i32){
        (x, (self.current.height as i32 - 1) - y)
    }

    pub fn get_pos(pos: &Transform) -> (i32, i32){
         Room::px_2_world(pos.translation().data[0], pos.translation().data[1])
    }
    
    // Convert from pixel coordinates 
    pub fn px_2_world(x: f32, y:f32) -> (i32, i32){
        ((((x - constants::TILE_SIZE) / constants::TILE_SIZE) as i32),
         (((y - constants::TILE_SIZE) / constants::TILE_SIZE) as i32)
        )
    }

    // Check to see if the resulting position is inside the map
    pub fn allowed_move(&mut self, pos: &Transform, horizontal: f32, vertical: f32, adj: Adj) -> bool{
        let (x, y) = Room::get_pos(pos);
        let north = (vertical > 0.)
            && ((y >= (self.current.height as i32 - constants::TILE_PER_PLAYER as i32))
                || colision(&adj.n));
        let east = (horizontal > 0.)
            && ((x >= (self.current.width as i32 - constants::TILE_PER_PLAYER as i32))
                || colision(&adj.e));
        let south = (vertical < 0.) && ((y == 0) || colision(&adj.s));
        let west = (horizontal < 0.) && ((x == 0) || colision(&adj.w));

        !north && !east && !south && !west
    }
    
    fn get_prop(&mut self, (x, y): (i32, i32), (xoff, yoff): (i32, i32)) -> Option<tiled::Properties> {
        
        // Bottom left
        if (x == 0 && xoff <= -1) || (y == 0 && yoff <= -1) {
            return None;  
        }
        
        if x + xoff > (self.current.width as i32 - constants::TILE_PER_PLAYER as i32) {
            return None;
        }

        if y + yoff > (self.current.height as i32 - constants::TILE_PER_PLAYER as i32) {
            return None;
        }
        
        let (x1, y1): (i32, i32) = self.world_2_tiled((x + xoff, y + yoff));
        let tile = self.current.layers[Layers::L4 as usize].tiles[y1 as usize][x1 as usize];

        match self.current.get_tileset_by_gid(tile.gid){
            Some(thing) => Some(thing.tiles[tile.gid as usize].properties.clone()),
            None => None,
        }
    }
    
    pub fn get_adj(&mut self, pos: &Transform) -> Adj {
        let (x, y): (i32, i32) = Room::get_pos(pos);
        
        Adj{
            cur: self.get_prop((x,y),(0,0)),
            n:   self.get_prop((x,y),(0,constants::TILE_PER_PLAYER as i32)),
            e:   self.get_prop((x,y),(constants::TILE_PER_PLAYER as i32,0)),
            s:   self.get_prop((x,y),(0, -constants::TILE_PER_PLAYER as i32)),
            w:   self.get_prop((x,y),(-constants::TILE_PER_PLAYER as i32,0)),
        }
    }
}
