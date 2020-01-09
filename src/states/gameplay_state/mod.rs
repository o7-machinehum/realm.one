use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera},
    window::ScreenDimensions,
    network::NetConnection,
    ecs::World,
};

use crate::map;
use crate::network;
use crate::components::PlayerComponent;
use crate::resources::ClientStatus;

pub struct GamePlayState {
    pub ip: String, // IP of server to connect to
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<PlayerComponent>();
        world.register::<map::Room>();

        let mut room = map::Room::new("resources/sprites/town.tmx".to_string());
        let status = ClientStatus::new();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);

        room.load_sprites(world);   // Load in all the sprites
        room.draw_room(world);      // Paint the initial room
        
        let player1 = PlayerComponent::new( 8.0, 8.0, (159, 147, 123, 135), &room.sprites);
        player1.insert(world);
        
        world.add_resource(status);
        world
            .create_entity()
            .with(room)
            .with(NetConnection::<String>::new(
                self.ip.parse().unwrap(),
            ))
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
