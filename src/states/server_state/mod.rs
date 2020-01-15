use amethyst::{
    prelude::*,
};

pub struct ServerState;

impl SimpleState for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // world.register::<PlayerComponent>();
        // world.register::<map::Room>();
        
        // Load in the world
        // let mut room = map::Room::new("resources/sprites/town.tmx".to_string());
        // room.load_sprites(world);   // Load in all the sprites
        
        // let player1 = PlayerComponent::new( 8.0, 8.0, (159, 147, 123, 135), &room.sprites);
        // player1.insert(world);
    
        // world
        //     .create_entity()
        //     .with(room)
        //     .with(NetConnection::<String>::new(
        //         self.ip.parse().unwrap(),
        //     ))
        //     .build();
    }
}
