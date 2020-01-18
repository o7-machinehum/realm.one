use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera},
    window::ScreenDimensions,
    network::NetConnection,
    ecs::World,
};

use crate::map;
use crate::components::{PlayerComponent, PlayerInfo, PlayerAction, PlayerList};
use crate::resources::ClientStatus;

pub struct GamePlayState {
    pub ip: String, // IP of server to connect to
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<PlayerComponent>();
        world.register::<map::TilePosition>();
        
        let sprites = map::SpritesContainer::new(&world, 371);
        let room = map::Room::new("resources/maps/townCompress.tmx".to_string());
        let player_list = PlayerList{ list: Vec::new() };

        let status = ClientStatus::new();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);
        
        // id 0 is always yourself
        let player1_info = PlayerInfo {
            id: 0,
            act: PlayerAction::Nothing,
            name: "Turnip".to_string(),
            room: "Room1".to_string(), 
            x: 8.0,        
            y: 8.0, 
            no: 318,        
            ea: 306, 
            so: 282,
            we: 294, 
        };
        
        let player1 = PlayerComponent::new(player1_info, &sprites.sprites);
        player1.insert(world);

        world.insert(status);
        world.insert(sprites);
        world.insert(room);
        world.insert(player_list);
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
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 10.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}
