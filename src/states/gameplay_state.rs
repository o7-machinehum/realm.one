use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera},
    window::ScreenDimensions,
    ecs::World,
};


use crate::map;
use crate::components::{PlayerComponent};
use crate::resources::{ClientStatus, PlayerList, IO, AppConfig, SpritesContainer};

pub struct GamePlayState {
    pub config: AppConfig,
}

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let io = IO::new();
        world.register::<PlayerComponent>();
        world.register::<map::TilePosition>();
        
        let sprites = SpritesContainer::new(&world);
        let room = map::Room::new("resources/maps/town.tmx".to_string());
        let player_list = PlayerList{ list: Vec::new() };

        let status = ClientStatus::new();

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);
        	
        world.insert(self.config.clone());
        world.insert(status);
        world.insert(sprites);
        world.insert(room);
        world.insert(player_list);
        world.insert(io);
    }

    // fn handle_event(
    //     &mut self,
    //     mut _data: StateData<'_, GameData<'_, '_>>,
    //     event: StateEvent,
    // ) -> SimpleTrans {
    //     if let StateEvent::Window(event) = &event {
    //         if let Some(event) = get_key(&event) {
    //             info!("handling key event: {:?}", event);
    //         }
    //     }
    //     // Keep going
    //     Trans::None
    // }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 10.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}
