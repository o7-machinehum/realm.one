use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
extern crate tiled;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use log::info;
use tiled::parse;

pub struct Room {
    pub current: tiled::Map,   // Current room 
    pub tiles: Vec<i32>,
    pub sprites: Vec<SpriteRender>,
}

// Comment
impl Room {
    pub fn new(fname: String) -> Self{
        let file = File::open(&Path::new(&fname)).unwrap();
    	let reader = BufReader::new(file);
        let mp =  parse(reader).unwrap();

        info!("{:?}", mp);

        Self{
            tiles: Room::count_tiles(&mp), 
            current: mp,
            sprites: Vec::new(), 
        }
    }
   
    fn count_tiles(mp: &tiled::Map) -> Vec<i32>{
        let mut v: Vec<i32> = Vec::new();
        for sets in &mp.tilesets{
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
               self.sprites.push(SpriteRender{
                   sprite_sheet: sheet_handle.clone(),
                   sprite_number: i as usize,
               });
           };
           ii += 1;
        };
    }
    
    pub fn load_room(&mut self, world: &mut World, dimensions: &ScreenDimensions) {
        let mut x = 0.0;
        let mut y = 0.0;

        for row in self.current.layers[0].tiles.iter().rev(){
            x = 0.0;
            y += 16.0;
            for col in row.iter(){
                x += 16.0; 
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
}
