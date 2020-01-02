use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteRender},
    window::ScreenDimensions,
};
use log::info;

use crate::map;
use crate::components::PlayerComponent;

pub struct GamePlayState {
    pub current_map: map::Room,
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<PlayerComponent>();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);

        self.current_map.load_sprites(world);             // Load in all the sprites
        self.current_map.draw_room(world, &dimensions);   // Paint the initial room
         
        // self.currentMap.load_obj(); 
        initialise_player(world, &self.current_map.sprites);
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


fn initialise_player(world: &mut World, sprite: &Vec<SpriteRender>) {
    let mut player1 = PlayerComponent::new( 64.0, 64.0 ); 

    let mut transform = Transform::default();
    transform.set_translation_xyz(player1.x, player1.y, 0.0); 

    // Create a player entity.
    world
        .create_entity()
        .with(sprite[125].clone())
        .with(player1)
        .with(transform)
        .build();
    
}


