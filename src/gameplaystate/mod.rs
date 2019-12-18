use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
};
use log::info;
pub mod map;

pub struct GamePlayState {
    pub currentMap: map::Room,
}

impl SimpleState for GamePlayState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        // let rm = tiles::Room::new("resources/sprites/first.tmx".to_string()).getInfo();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        
        let sprites = load_sprites(world, &self.currentMap);
        // init_sprites(world, &sprites, &dimensions);
        load_room(world, &sprites, &dimensions, &self.currentMap, 0);
        load_room(world, &sprites, &dimensions, &self.currentMap, 1);
        load_room(world, &sprites, &dimensions, &self.currentMap, 2);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World, mp: &map::Room) -> Vec<SpriteRender> {
    let mut sprites = vec![];
    let mut ii = 0;

    for sets in &mp.current.tilesets {
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
        
       for i in (0..mp.tiles[ii]){ 
           sprites.push(SpriteRender{
               sprite_sheet: sheet_handle.clone(),
               sprite_number: i as usize,
           });
       };
       ii += 1;
    };
    sprites 
}

fn init_sprites(world: &mut World, sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
    for (i, sprite) in sprites.iter().enumerate() {
        // Center our sprites around the center of the window
        let x = (i as f32 - 1.) * 100. + dimensions.width() * 0.5;
        let y = (i as f32 - 1.) * 100. + dimensions.height() * 0.5;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);

        // Create an entity for each sprite and attach the `SpriteRender` as
        // well as the transform. If you want to add behaviour to your sprites,
        // you'll want to add a custom `Component` that will identify them, and a
        // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
        world
            .create_entity()
            .with(sprite.clone())
            .with(transform)
            .build();
    }
}

fn load_room(world: &mut World, sprites: &[SpriteRender], dimensions: &ScreenDimensions, room: &map::Room, layer: usize) {
    let mut x = 0.0;
    let mut y = 0.0;

    for row in room.current.layers[layer].tiles.iter().rev(){
        x = 0.0;
        y += 16.0;
        for col in row.iter(){
            x += 16.0; 
            let mut transform = Transform::default();
            transform.set_translation_xyz(x, y, 0.);
            
            info!("Col: {:?}, Row: {:?}", col, row);
            
            if(*col != 0){
                world
                    .create_entity()
                    .with(sprites[*col as usize - 1].clone())
                    .with(transform)
                    .build();
            } 
        }
    }
}
