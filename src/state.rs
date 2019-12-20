use amethyst::{
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, prelude::*,
    renderer::{Camera},
    window::ScreenDimensions,
};
use log::info;

use crate::map;
use crate::obj::Location;

pub struct GamePlayState {
    pub current_map: map::Room,
}


impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
       
        // Example of creating an entity with a location.
        //world.register::<Location>();
        //let player = world.create_entity()
        //                  .with(Location::new(0,0,0))
        //                  .build();
        //{ 
        //    let mut storage = world.write_storage::<Location>();
        //    let mut my = storage.get_mut(player).unwrap();
        //    println!("{:?}", my.x);
        //} 
        

        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();
        init_camera(world, &dimensions);

        self.current_map.load_sprites(world);             // Load in all the sprites
        self.current_map.load_room(world, &dimensions);   // Paint the initial room
         
        // self.currentMap.load_obj(); 
         
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
        }

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
