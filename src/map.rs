use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

extern crate tiled;
use std::{
    fs::File,
    io::BufReader,
    path::Path,
};
use log::info;

enum Layers {
    L1Static = 0,
    L2Static,
}

pub struct Room {
    pub current: tiled::Map,   // Current room 
    pub tiles: Vec<i32>,
    pub sprites: Vec<SpriteRender>,
}

// Comment
impl Room {
    pub fn new(file_name: String) -> Self {
        let file = File::open(&Path::new(&file_name)).unwrap();
    	let reader = BufReader::new(file);
        let map =  tiled::parse(reader).unwrap();

        // info!("{:?}", map);

        Self {
            tiles: Room::count_tiles(&map), 
            current: map,
            sprites: Vec::new(), 
        }
    }
   
    fn count_tiles(map: &tiled::Map) -> Vec<i32> {
        let mut v: Vec<i32> = Vec::new();
        for sets in &map.tilesets {
            v.push((sets.images[0].width / sets.tile_width as i32) * (sets.images[0].height / sets.tile_height as i32)) 
        }

        info!("Tiles in the images: {:?}", v);

        v
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
 
            for i in 0..self.tiles[ii] { 
                self.sprites.push(SpriteRender {
                    sprite_sheet: sheet_handle.clone(),
                    sprite_number: i as usize,
                });
            };
            ii += 1;
        };
    }

    fn draw_layer(&mut self, world: &mut World, layer: Layers) {
        let mut x;
        let mut y = 0.0;

        const TILE_SIZE : f32 = 8.0;

        for row in self.current.layers[layer as usize].tiles.iter().rev() {
            x = 0.0;
            y += TILE_SIZE;

            for col in row.iter() {
                x += TILE_SIZE; 

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
        self.draw_layer(world, Layers::L2Static);
        self.draw_layer(world, Layers::L1Static);
    } 
}
