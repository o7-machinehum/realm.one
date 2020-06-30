use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

#[derive(Default)]
pub struct SpritesContainer {
    pub sprites: Vec<SpriteRender>,
    pub text: Vec<SpriteRender>,
    pub background: Vec<SpriteRender>,
}

fn load(world: &World, img: String, ron: String, num_sprites: u32) -> Vec<SpriteRender> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        
        loader.load(
            img,
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    
        loader.load(
            ron,
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
    container
}

impl SpritesContainer {
    pub fn new(world: &World) -> Self {
        Self {
            sprites: load(world,
                          "sprites/master16.png".to_string(), 
                          "sprites/master16.ron".to_string(),
                          1068),
            
            text: load(world,
                       "fonts/text.png".to_string(),
                       "fonts/text.ron".to_string(),
                       225),
            
            background: load(world,
                       "background/background.png".to_string(),
                       "background/background.ron".to_string(),
                       1),
        }
    }
}
