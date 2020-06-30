use amethyst::{
    core::transform::Transform, ecs::World, prelude::*, renderer::Camera, window::ScreenDimensions,
    renderer::{SpriteRender},
};

use crate::components::LifeformComponent;
use crate::map;
use crate::resources::{AppConfig, CommandQueue, LifeformList, SpritesContainer};
use log::info;

pub struct GamePlayState {
    pub config: AppConfig,
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<LifeformComponent>();
        world.register::<map::TilePosition>();
        let sprites = SpritesContainer::new(&world);
        let room = map::Room::new("resources/maps/town.tmx".to_string(), false);
        let player_list = LifeformList::new();
        let command_queue = CommandQueue::new();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);
        draw_background(world, &sprites.background[0], &dimensions);

        world.insert(self.config.clone());
        world.insert(sprites);
        world.insert(room);
        world.insert(player_list);
        world.insert(command_queue);
    }
}

fn draw_background(world: &mut World, background: &SpriteRender, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    let width = dimensions.width() as f64 / dimensions.hidpi_factor();
    let height = dimensions.height() as f64 / dimensions.hidpi_factor();
    transform.set_translation_xyz(width as f32 * 0.5, height as f32 * 0.5, -10.);
    
    world
        .create_entity()
        .with(background.clone())
        .with(transform)
        .build();

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
