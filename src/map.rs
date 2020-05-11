use amethyst::{
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Entity, FlaggedStorage},
};

extern crate tiled;
use std::{fs::File, io::BufReader, path::Path};

use crate::components::{Monster, Orientation};
use crate::constants;
use log::info;

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Layers {
    L1 = 0,
    L2,
    L3,
    L4,
    L5,
    L6,
    L7,
}

pub struct Room {
    pub map: tiled::Map,
    pub xsize: usize,
    pub tile_ent: Vec<Entity>,
    pub update: bool,
    pub name: String,
    pub monsters: Vec<Monster>,
}

impl Default for Room {
    fn default() -> Self {
        let file_name = "resources/maps/first.tmx".to_string();
        Room::new(file_name, false)
    }
}

impl Room {
    pub fn new(file_name: String, server: bool) -> Self {
        let file = File::open(&Path::new(&file_name)).unwrap();
        let reader = BufReader::new(file);
        let map =
            tiled::parse_with_path(reader, &Path::new("resources/sprites/master16.tsx")).unwrap();
        let monsters = match server {
            true => Room::get_monsters(&map),
            false => Vec::<Monster>::new(),
        };

        Self {
            xsize: map.layers[0].tiles[0].len() - 1,
            map,
            tile_ent: Vec::new(),
            update: true,
            name: file_name,
            monsters,
        }
    }

    pub fn change(&mut self, map_name: String) {
        let file = File::open(&Path::new(&map_name)).unwrap();
        let reader = BufReader::new(file);
        let map =
            tiled::parse_with_path(reader, &Path::new("resources/sprites/master16.tsx")).unwrap();
        self.map = map;
        self.update = true;
    }

    // Convert world coordinates to tiled coordinates
    fn world_2_tiled(&self, (x, y): (i32, i32)) -> (i32, i32) {
        (x, (self.map.height as i32 - 1) - y)
    }

    pub fn get_pos(pos: &Transform) -> (i32, i32) {
        Room::px_2_world(pos.translation().data[0], pos.translation().data[1])
    }

    // Convert from pixel coordinates
    pub fn px_2_world(x: f32, y: f32) -> (i32, i32) {
        (
            (((x - constants::TILE_SIZE) / constants::TILE_SIZE) as i32),
            (((y - constants::TILE_SIZE) / constants::TILE_SIZE) as i32),
        )
    }

    // Check to see if the resulting position is inside the map
    pub fn allowed_move(&self, pos: &Transform, facing: &Orientation) -> bool {
        let adj: Adj = Adj::new(self, pos); // Get all the adjasent tiles
        let (x, y) = Room::get_pos(pos);    // Get x/y coord of transform

        let north = (*facing == Orientation::North)
            && ((y >= (self.map.height as i32 - constants::TILE_PER_PLAYER as i32))
                || colision(&adj.n));
        let east = (*facing == Orientation::East)
            && ((x >= (self.map.width as i32 - constants::TILE_PER_PLAYER as i32))
                || colision(&adj.e));
        let south = (*facing == Orientation::South) && ((y == 0) || colision(&adj.s));
        let west = (*facing == Orientation::West) && ((x == 0) || colision(&adj.w));

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
        info!("{}, {}", x1, y1);
        let tile = self.map.layers[Layers::L4 as usize].tiles[y1 as usize][x1 as usize];

        match self.map.get_tileset_by_gid(tile.gid) {
            Some(thing) => Some(thing.tiles[tile.gid as usize].properties.clone()),
            None => None,
        }
    }

    fn get_monsters(map: &tiled::Map) -> Vec<Monster> {
        let mut monsters = Vec::<Monster>::new();
        // let tile = self.map.layers[Layers::L7 as usize].tiles[y1 as usize][x1 as usize];
        for (x, row) in map.layers[Layers::L7 as usize]
            .tiles
            .iter()
            .rev()
            .enumerate()
        {
            for (y, col) in row.iter().enumerate() {
                if col.gid != 0 {
                    let prop = map.get_tileset_by_gid(col.gid).unwrap().tiles[col.gid as usize]
                        .properties
                        .clone();
                    info!("{:?}", prop);
                    monsters.push(Monster::new(prop, (x as u32 + 1, y as u32 + 1)));
                }
            }
        }
        monsters
    }

    
}

pub struct Adj {
    pub cur: Option<tiled::Properties>,
    pub n: Option<tiled::Properties>,
    pub e: Option<tiled::Properties>,
    pub s: Option<tiled::Properties>,
    pub w: Option<tiled::Properties>,
}

impl Adj {
    pub fn new(map: &Room, pos: &Transform) -> Self {
        let (x, y): (i32, i32) = Room::get_pos(pos);
        

        Self {
            cur: map.get_prop((x, y), (0, 0)),
            n: map.get_prop((x, y), (0, constants::TILE_PER_PLAYER as i32)),
            e: map.get_prop((x, y), (constants::TILE_PER_PLAYER as i32, 0)),
            s: map.get_prop((x, y), (0, -constants::TILE_PER_PLAYER as i32)),
            w: map.get_prop((x, y), (-constants::TILE_PER_PLAYER as i32, 0)),
        }
    }
}


impl Component for TilePosition {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

pub struct TilePosition {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub gid: usize,
}

impl TilePosition {
    pub fn new(x: usize, y: usize, z: usize, gid: usize) -> Self {
        Self { x, y, z, gid }
    }

    pub fn to_trans(&mut self) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            (self.x as f32 * constants::TILE_SIZE) as f32 + 8.0,
            (self.y as f32 * constants::TILE_SIZE) as f32 + 8.0,
            self.z as f32 * 0.1,
        );
        transform
    }
}

pub fn colision(tile: &Option<tiled::Properties>) -> bool {
    match tile {
        None => return false,
        Some(i) => match i.get("Collision") {
            Some(value) => match value {
                tiled::PropertyValue::BoolValue(val) => return *val, 
                _ => (),
            },
            None => return false,
        },
    }
    return true
}
