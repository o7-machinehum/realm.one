use amethyst::{
    assets::Handle,
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteSheet},
    window::ScreenDimensions,
};

use crate::map;
use crate::components::PlayerComponent;
use crate::character_sprites::{get_oriented_sprite, load_sprites};

pub struct GamePlayState {}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<PlayerComponent>();
        world.register::<map::Room>();

        let mut room = map::Room::new("resources/sprites/town.tmx".to_string());

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);

        room.load_sprites(world);   // Load in all the sprites
        room.draw_room(world);      // Paint the initial room
        
        let character_spritesheet_handle = load_sprites(world);

        initialise_player(world, character_spritesheet_handle);
    
        world
            .create_entity()
            .with(room)
            .build();
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


fn initialise_player(world: &mut World, spritesheet_handle: Handle<SpriteSheet>) {
    let player1 = PlayerComponent::new( 8.0, 8.0, spritesheet_handle.clone() ); 

    let mut transform = Transform::default();
    transform.set_translation_xyz(player1.x, player1.y, 0.0); 

    // Create a player entity.
    world
        .create_entity()
        .with(get_oriented_sprite(spritesheet_handle, player1.orientation.clone()))
        .with(player1)
        .with(transform)
        .build();
    
}


