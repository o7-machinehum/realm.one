use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};

extern crate tiled;
use std::{
    fs::File,
    io::BufReader,
    path::Path,
};

use log::info;
use crate::constants;

#[derive(Clone, Debug)]
enum Layers {
    L1 = 0,
    L2,
    L3,
    L4,
    L5,
    L6,
}

#[derive(Default)]
pub struct SpritesContainer {
    pub sprites: Vec<SpriteRender>,
}

impl SpritesContainer {
    pub fn new(world: &World, num_sprites: u32) -> Self {
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            
            loader.load(
                "sprites/master16.png",
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };
    
        let sheet_handle = {
            let loader = world.read_resource::<Loader>();
            let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    
            loader.load(
                "sprites/master16.ron",
                SpriteSheetFormat(texture_handle),
                (),
                &sheet_storage,
            )
        };
        
        let mut container: Vec<SpriteRender> = Vec::new();
    
        for i in 0..num_sprites { 
            container.push(SpriteRender {
                sprite_sheet: sheet_handle.clone(),
                sprite_number: i as usize,
            });
        };
        
        Self {
            sprites: container,
        }
    }
}

pub struct Room {
    map: tiled::Map,
    xsize: usize,
}

impl Default for Room {
    fn default() -> Self { 
        let file = File::open(&Path::new(&"resources/maps/first.tmx".to_string())).unwrap();
        let reader = BufReader::new(file);
        let map =  tiled::parse_with_path(reader, &Path::new("resources/sprites/master16.tsx")).unwrap();
        
        Self { 
            xsize: map.layers[0].tiles[0].len() - 1,
            map,
        }
    }
}

impl Room {
    pub fn new(file_name: String) -> Self {
        let file = File::open(&Path::new(&file_name)).unwrap();
    	let reader = BufReader::new(file);
        let map =  tiled::parse_with_path(reader, &Path::new("resources/sprites/master16.tsx")).unwrap();

        Self {
            xsize: map.layers[0].tiles[0].len() - 1,
            map, 
        }
    }

    pub fn change(&mut self, map: tiled::Map) {
        info!("Inserting New Map");
        self.map = map;
    }

    fn get_gid(&self, loc: &TilePosition) -> u32 {
        self.map.layers[loc.z].tiles[self.xsize - loc.x][loc.y].gid
    }
    
    /// Compares the gid of the tile in a current location to that of a room
    /// If the tile isn't the same, it returns Some(usize) of the proper tile
    pub fn diff_gid(&self, loc: &TilePosition) -> Option<usize> {
        let gid =  self.get_gid(loc);
            
        if loc.gid as u32 != gid {
           return Some(gid as usize);
        }
        None 
    }

    fn draw_layer(&mut self, world: &mut World, layer: Layers, sprites: &SpritesContainer) {
        for (x, row) in self.map.layers[layer.clone() as usize].tiles.iter().rev().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if col.gid != 0 {
                    let mut loc = TilePosition::new(x, y, layer.clone() as usize, col.gid as usize - 1);
                    let mut transform = loc.to_trans(); 
                    
                    world
                        .create_entity()
                        .with(sprites.sprites[loc.gid].clone()) 
                        .with(transform)
                        .with(loc) 
                        .build();
                }
            }
        }
    }
    
    pub fn draw_room(&mut self, world: &mut World, sprites: &SpritesContainer) {
        self.draw_layer(world, Layers::L5, sprites);
        self.draw_layer(world, Layers::L4, sprites);
        self.draw_layer(world, Layers::L3, sprites);
        self.draw_layer(world, Layers::L2, sprites);
        self.draw_layer(world, Layers::L1, sprites);
    }
}

pub struct Adj {
    pub cur: Option<tiled::Properties>,
    pub n: Option<tiled::Properties>,
    pub e: Option<tiled::Properties>,
    pub s: Option<tiled::Properties>,
    pub w: Option<tiled::Properties>,
}

impl Component for TilePosition{
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl Component for SpritesContainer{
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

pub struct TilePosition {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub gid: usize,
}

impl TilePosition {
    pub fn new(x: usize, y:usize, z: usize, gid: usize) -> Self {
        Self {
            x,
            y,
            z,
            gid,
        }
    }
    
    // pub fn to_spr_num(&mut self) -> usize {
    //     (self.x * self.y)
    // }

    pub fn to_trans(&mut self) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz((self.x as f32 * constants::TILE_SIZE) as f32, 
                                      (self.y as f32 * constants::TILE_SIZE) as f32, 
                                      (self.z as f32)
                                     );
        transform
    }
}

// Comment
// impl Room {
//    pub fn change(&mut self, newMap: tiled::Map) {
//        self.len_width = Room::count_tiles(&newMap); 
//        self.current = newMap;
//    }
//
//    fn count_tiles(map: &tiled::Map) -> Vec<i32> {
//        let mut v: Vec<i32> = Vec::new();
//        for sets in &map.tilesets {
//            v.push((sets.images[0].width / sets.tile_width as i32) * (sets.images[0].height / sets.tile_height as i32)) 
//        }
//
//        info!("Tiles in the images: {:?}", v);
//
//        v
//    }
//
    
//}
//
//    
//
//    // Convert world coordinates to tiled coordinates
//    fn world_2_tiled(&mut self, (x, y): (i32, i32)) -> (i32, i32){
//        (x, (self.current.height as i32 - 1) - y)
//    }
//
//    pub fn get_pos(pos: &Transform) -> (i32, i32){
//         Room::px_2_world(pos.translation().data[0], pos.translation().data[1])
//    }
//    
//    // Convert from pixel coordinates 
//    pub fn px_2_world(x: f32, y:f32) -> (i32, i32){
//        ((((x - constants::TILE_SIZE) / constants::TILE_SIZE) as i32),
//         (((y - constants::TILE_SIZE) / constants::TILE_SIZE) as i32)
//        )
//    }
//
//    // Check to see if the resulting position is inside the map
//    pub fn allowed_move(&mut self, pos: &Transform, horizontal: f32, vertical: f32, adj: Adj) -> bool{
//        let (x, y) = Room::get_pos(pos);
//        let north = (vertical > 0.)
//            && ((y >= (self.current.height as i32 - constants::TILE_PER_PLAYER as i32))
//                || colision(&adj.n));
//        let east = (horizontal > 0.)
//            && ((x >= (self.current.width as i32 - constants::TILE_PER_PLAYER as i32))
//                || colision(&adj.e));
//        let south = (vertical < 0.) && ((y == 0) || colision(&adj.s));
//        let west = (horizontal < 0.) && ((x == 0) || colision(&adj.w));
//
//        !north && !east && !south && !west
//    }
//    
//    fn get_prop(&mut self, (x, y): (i32, i32), (xoff, yoff): (i32, i32)) -> Option<tiled::Properties> {
//        
//        // Bottom left
//        if (x == 0 && xoff <= -1) || (y == 0 && yoff <= -1) {
//            return None;  
//        }
//        
//        if x + xoff > (self.current.width as i32 - constants::TILE_PER_PLAYER as i32) {
//            return None;
//        }
//
//        if y + yoff > (self.current.height as i32 - constants::TILE_PER_PLAYER as i32) {
//            return None;
//        }
//        
//        let (x1, y1): (i32, i32) = self.world_2_tiled((x + xoff, y + yoff));
//        let tile = self.current.layers[Layers::L4 as usize].tiles[y1 as usize][x1 as usize];
//
//        match self.current.get_tileset_by_gid(tile.gid){
//            Some(thing) => Some(thing.tiles[tile.gid as usize].properties.clone()),
//            None => None,
//        }
//    }
//    
//    pub fn get_adj(&mut self, pos: &Transform) -> Adj {
//        let (x, y): (i32, i32) = Room::get_pos(pos);
//        
//        Adj{
//            cur: self.get_prop((x,y),(0,0)),
//            n:   self.get_prop((x,y),(0,constants::TILE_PER_PLAYER as i32)),
//            e:   self.get_prop((x,y),(constants::TILE_PER_PLAYER as i32,0)),
//            s:   self.get_prop((x,y),(0, -constants::TILE_PER_PLAYER as i32)),
//            w:   self.get_prop((x,y),(-constants::TILE_PER_PLAYER as i32,0)),
//        }
//    }
//}
