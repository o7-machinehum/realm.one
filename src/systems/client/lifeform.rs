use amethyst::{ 
    core::{Transform, SystemDesc, bundle::SystemBundle},
    derive::SystemDesc,
    ecs::{WriteStorage, Entities, World, Read, System, SystemData, DispatcherBuilder, Join},
    renderer::{SpriteRender, resources::Tint},
    shrev::{EventChannel, ReaderId},
    Result, 
};

use log::info;

use crate::{ 
    components::{LifeformComponent},
};

pub enum LifeformEvent {
    UpdatePlayer(LifeformComponent),
    RemovePlayer(u64),
}

#[derive(SystemDesc)]
pub struct LifeformSystem {
    event_reader: ReaderId<LifeformEvent>,
}


pub struct LifeformSystemBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for LifeformSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            LifeformSystemDesc::default().build(world),
            "lifeform_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct LifeformSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, LifeformSystem> for LifeformSystemDesc {
    fn build(self, world: &mut World) -> LifeformSystem {
        <LifeformSystem as System<'_>>::SystemData::setup(world);
        let event_reader = world
            .fetch_mut::<EventChannel<LifeformEvent>>()
            .register_reader();
        LifeformSystem{ event_reader }
    }
}

impl<'s> System<'s> for LifeformSystem {
    type SystemData = (
        Read <'s, EventChannel<LifeformEvent>>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, LifeformComponent>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        Entities<'s>,
    );
 
    fn run(&mut self, (events, mut transforms, mut players, mut sprite_renders, mut tints, entities): Self::SystemData) {
        for event in events.read(&mut self.event_reader) {
            match event{
                LifeformEvent::UpdatePlayer(new) => {
                    for (transform, player, sprite_render, tint) in (&mut transforms, &mut players, &mut sprite_renders, &mut tints).join() { 
                        if player.id() == new.id() {
                            info!("Updating Player: {:?}", player);

                            if *player.trans().translation() != new.xyz() { 
                                transform.set_translation(new.xyz()); 
                            }

                            if player.orientation != new.orientation {
                                sprite_render.sprite_number = new.get_dir();
                            }

                            if player.hp != new.hp {
                                // oh damn we hurtin
                                *tint = Tint(new.tint());  
                            }
                            
                            *player = new.clone();
                        }
                    }        
                }, 
                LifeformEvent::RemovePlayer(uid) => {
                    info!("Removing Player of id: {}", uid);
                    for (e, player) in (&*entities, &mut players).join() { 
                        if player.id() == *uid {
                            entities.delete(e).expect("Failed to delete old player entities");
                        }
                    } 
                } 
            }
        }
    }
}
