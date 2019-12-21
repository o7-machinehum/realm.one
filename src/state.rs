use amethyst::{
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, prelude::*,
    renderer::{Camera, SpriteRender},
    window::ScreenDimensions,
    ecs::{Component, DenseVecStorage, FlaggedStorage},
};
use log::info;

use crate::map;

pub struct GamePlayState {
    pub current_map: map::Room,
}

pub struct Player{
    pub x: f32,
    pub y: f32,
}

impl Player {
    fn new( x: f32, y: f32 ) -> Player {
        Player {
            x,
            y, 
        }
    }
}

impl Component for Player{
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Player>();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);

        self.current_map.load_sprites(world);             // Load in all the sprites
        self.current_map.load_room(world, &dimensions);   // Paint the initial room
         
        // self.currentMap.load_obj(); 
        initialise_player(world, &self.current_map.sprites);         
    }

    //fn handle_event(
    //    &mut self,
    //    mut _data: StateData<'_, GameData<'_, '_>>,
    //    event: StateEvent,
    //) -> SimpleTrans {
    //    if let StateEvent::Window(event) = &event {
    //        // Check if the window should be closed
    //        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
    //            return Trans::Quit;
    //        }

    //        // Listen to any key events
    //        if let Some(event) = get_key(&event) {
    //            // info!("handling key event: {:?}", event);
    //        }
    //    }

    //    Trans::None
    //}
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


fn initialise_player(world: &mut World, sprite: &Vec<SpriteRender>) {
    let mut transform = Transform::default();
    let mut p1 = Player::new( 64.0, 64.0 ); 
    // Correctly position the paddles.
    transform.set_translation_xyz(p1.x, p1.y, 0.0); 

    // Create a player entity.
    world
        .create_entity()
        .with(sprite[125].clone())
        .with(p1)
        .with(transform)
        .build();
    
}
