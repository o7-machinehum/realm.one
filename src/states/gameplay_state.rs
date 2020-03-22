use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera},
    window::ScreenDimensions,
    ecs::World,
};

use log::info;
use crate::map;
use crate::components::{LifeformComponent};
use crate::resources::{ClientStatus, LifeformList, IO, AppConfig, SpritesContainer, Input};

pub struct GamePlayState {
    pub config: AppConfig,
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let io = IO::new();
        world.register::<LifeformComponent>();
        world.register::<map::TilePosition>();
        
        let sprites = SpritesContainer::new(&world);
        let room = map::Room::new("resources/maps/town.tmx".to_string());
        let player_list = LifeformList::new();
        let inputs = Input::new();

        let status = ClientStatus::new();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);
        	
        world.insert(self.config.clone());
        world.insert(status);
        world.insert(sprites);
        world.insert(room);
        world.insert(player_list);
        world.insert(io);
        world.insert(inputs);
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    let width = dimensions.width() as f64 / dimensions.hidpi_factor();
    let height = dimensions.height() as f64 / dimensions.hidpi_factor();

    info!("{:?}", dimensions);
    transform.set_translation_xyz(width as f32 * 0.5, height as f32 * 0.5, 10.);

    world
        .create_entity()
        .with(Camera::standard_2d(width as f32, height as f32))
        .with(transform)
        .build();
}
