use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ecs::{Component, DenseVecStorage, FlaggedStorage, Entity},
};

extern crate tiled;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
    fs,
};

use log::info;
use crate::constants;
use crate::mech::{colision};
use stringreader::StringReader;

#[derive(Clone, Debug)]
#[allow(dead_code)]
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

// MapList for the server
pub struct MapList {
    pub list: Vec<Room>,
}

impl Default for MapList {
    fn default() -> Self {
        Self{ list: Vec::new(), } 
    }
}

impl MapList {
    pub fn add(&mut self, file_name: String) {
        self.list.push(Room::new(file_name));
    }
}

pub struct Room {
    pub map: tiled::Map,
    pub xsize: usize,
    pub tile_ent: Vec<Entity>,
    pub update: bool,
    pub raw: String,
    pub name: String,
}

impl Default for Room {
    fn default() -> Self { 
        let file_name =  "resources/maps/first.tmx".to_string();
        let mut file = File::open(&Path::new(&file_name)).unwrap();
        let reader = BufReader::new(file);
        let map =  tiled::parse_with_path(reader, &Path::new("resources/sprites/master16.tsx")).unwrap();
        
        Self { 
            xsize: map.layers[0].tiles[0].len() - 1,
            map,
            tile_ent: Vec::new(),
            update: true,
            raw: fs::read_to_string(file_name.clone()).unwrap(),
            name: file_name,
        }
    }
}

impl Room {
    pub fn new(file_name: String) -> Self {
        let mut file = File::open(&Path::new(&file_name)).unwrap();
        let reader = BufReader::new(file);
        let map =  tiled::parse_with_path(reader, &Path::new("resources/sprites/master16.tsx")).unwrap();

        Self {
            xsize: map.layers[0].tiles[0].len() - 1,
            map, 
            tile_ent: Vec::new(),
            update: true,
            raw: fs::read_to_string(file_name.clone()).unwrap(),
            name: file_name,
        }
    }

    pub fn change(&mut self, map_name: String, map_data: String) {
        info!("Loading the map: {}!", map_name);
        let streader = StringReader::new(&map_data);     // Make a buffer
        let reader = BufReader::new(streader);
        let map =  tiled::parse_with_path(reader, &Path::new("resources/sprites/master16.tsx")).unwrap();
        
        self.map = map;
        self.update = true;
    }
    
    // Convert world coordinates to tiled coordinates
    fn world_2_tiled(&self, (x, y): (i32, i32)) -> (i32, i32){
        (x, (self.map.height as i32 - 1) - y)
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
    pub fn allowed_move(&self, pos: &Transform, horizontal: f32, vertical: f32, adj: Adj) -> bool {
        let (x, y) = Room::get_pos(pos);
        let north = (vertical > 0.)
            && ((y >= (self.map.height as i32 - constants::TILE_PER_PLAYER as i32))
                || colision(&adj.n));
        let east = (horizontal > 0.)
            && ((x >= (self.map.width as i32 - constants::TILE_PER_PLAYER as i32))
                || colision(&adj.e));
        let south = (vertical < 0.) && ((y == 0) || colision(&adj.s));
        let west = (horizontal < 0.) && ((x == 0) || colision(&adj.w));

        !north && !east && !south && !west
    }
    
    fn get_prop(&self, (x, y): (i32, i32), (xoff, yoff): (i32, i32)) -> Option<tiled::Properties> {
        
        // Bottom left
        if (x == 0 && xoff <= -1) || (y == 0 && yoff <= -1) {
            return None;  
        }
        
        if x + xoff > (self.map.width as i32 - constants::TILE_PER_PLAYER as i32) {
            return None;
        }

        if y + yoff > (self.map.height as i32 - constants::TILE_PER_PLAYER as i32) {
            return None;
        }
        
        let (x1, y1): (i32, i32) = self.world_2_tiled((x + xoff, y + yoff));
        let tile = self.map.layers[Layers::L4 as usize].tiles[y1 as usize][x1 as usize];

        match self.map.get_tileset_by_gid(tile.gid){
            Some(thing) => Some(thing.tiles[tile.gid as usize].properties.clone()),
            None => None,
        }
    }
    
    pub fn get_adj(&self, pos: &Transform) -> Adj {
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

    pub fn to_trans(&mut self) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz((self.x as f32 * constants::TILE_SIZE) as f32 + 8.0, 
                                      (self.y as f32 * constants::TILE_SIZE) as f32 + 8.0, 
                                      self.z as f32 * 0.1
                                     );
        transform
    }
}
