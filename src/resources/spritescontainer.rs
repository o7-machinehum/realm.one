use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

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
