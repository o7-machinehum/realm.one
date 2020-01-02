use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

#[derive(Clone)]
pub enum Orientation {
    South,
    West,
    East,
    North,
}

// Temporary solution, I think that in the future we should have "prefabs" where the spritesheet
// and sprites positions (i.e. 5, 17, 29, 41) defined for each entity, and then a generic
// Orientation enum / component?
impl Orientation {
    fn value(&self) -> usize {
        match *self {
            Orientation::South => 5,
            Orientation::West => 17,
            Orientation::East => 29,
            Orientation::North => 41,
        }
    }
}

pub fn load_sprites(world: &mut World) -> Handle<SpriteSheet> {
    // Load the images.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            format!("sprites/{}.png", "characters"), 
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the images' metadata.
    let loader = world.read_resource::<Loader>();
    let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        format!("sprites/{}.ron", "characters"), 
        SpriteSheetFormat(texture_handle),
        (),
        &sheet_storage,
    )
}

pub fn get_oriented_sprite(sheet_handle: Handle<SpriteSheet>, orientation: Orientation) -> SpriteRender {
    SpriteRender {
        sprite_sheet: sheet_handle,
        sprite_number: orientation.value(),
    }
}

