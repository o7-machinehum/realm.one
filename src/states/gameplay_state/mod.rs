use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera},
    window::ScreenDimensions,
    network::NetConnection,
    ecs::World,
    shrev::EventChannel
};

use log::info;
use crate::map;
use crate::components::PlayerComponent;
use crate::resources::ClientStatus;
use crate::events::{Events};

pub struct GamePlayState {
    pub ip: String, // IP of server to connect to
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<PlayerComponent>();
        world.register::<map::TilePosition>();
        world.register::<map::SpritesContainer >();
        
        let mut sprites = map::SpritesContainer::new(&world, 371);
        let mut room = map::Room::new("resources/maps/townCompress.tmx".to_string());
        room.draw_room(world, &sprites);

        let status = ClientStatus::new();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);

        // let player1 = PlayerComponent::new( 8.0, 8.0, (318, 306, 282, 294), &room.sprites);
        // player1.insert(world);

        let mut mapEvents = EventChannel::<Events>::new();
        mapEvents.register_reader();

        world.insert(status);
        world.insert(mapEvents);
        world.insert(sprites);
        world.insert(room);
        world
            .create_entity()
            .with(NetConnection::<Vec<u8>>::new(
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
