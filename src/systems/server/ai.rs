use amethyst::{
    core::{SystemDesc, bundle::SystemBundle},
    derive::SystemDesc,
    ecs::{Write, World, Read, System, SystemData, DispatcherBuilder},
    shrev::{EventChannel},
    Result, 
};

use std::time::Instant;

use crate::{
    resources::{LifeformList, MapList},
    components::{LifeformType, Action, LifeformComponent, get_rand_orientation},
    systems::server::{LifeformEvent},
};

#[allow(unused_imports)]
use log::info;

/// Events that pertain to the Ai System

#[derive(SystemDesc)]
pub struct AiSystem {
    timer: Instant,
}

pub struct AiSystemBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for AiSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            AiSystemDesc::default().build(world),
            "ai_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct AiSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, AiSystem> for AiSystemDesc {
    fn build(self, world: &mut World) -> AiSystem {
        <AiSystem as System<'_>>::SystemData::setup(world);
        AiSystem { timer: Instant::now(), }
    }
}

impl AiSystem {
    fn get_monster_action(
        &self, 
        monster: LifeformComponent, 
        players: &Vec<u64>,
        lf: &LifeformList
    ) -> Option<LifeformEvent>
    {
        for p in players.iter() {
            if let Some(player) = lf.get_from_id(*p) {
                if monster.in_range(&player) {
                   
                    // If in front, attack
                    if monster.is_in_front(&player) {
                        return Some(LifeformEvent::Action(
                            Action::Melee,
                            monster.clone()
                            )
                        )
                    }
                    
                    // If adjasent, rotate
                    match monster.is_adjasent(&player) {
                        Some(direction) => {
                            return Some(LifeformEvent::Action(
                                Action::Rotate(direction), 
                                monster.clone()
                                )
                            )
                        },
                        None =>()
                    }
                    
                    // If just in range, walk towards
                    return Some(LifeformEvent::Action(
                        Action::Move(monster.direction_towards(&player)),
                        monster.clone()
                        )
                    )
                }
            }
        }
        
        // If nothing else, just randomly wander around
        Some(LifeformEvent::Action(Action::Move(get_rand_orientation()), monster.clone()))
    }

    fn get_all_monster_actions(
        &self, 
        monsters: &Vec<u64>, 
        players: &Vec<u64>,
        lf: &LifeformList
    ) -> Vec<LifeformEvent>
    {
        let mut events = Vec::<LifeformEvent>::new();

        for monster in monsters {
            match self.get_monster_action(lf.get_from_id(*monster).unwrap(), players, lf) {
                Some(act) => events.push(act),
                None => (),
            }
        }
        
        events
    }
}

impl<'a> System<'a> for AiSystem {
    type SystemData = (
        Write<'a, EventChannel<LifeformEvent>>,
        Read <'a, MapList>,
        Read <'a, LifeformList>,
    );

    fn run(&mut self, (mut actions, maps, lifeforms): Self::SystemData) {
        let now = Instant::now();

        if now.duration_since(self.timer).as_millis() >= 1000 {
            self.timer = now.clone();

            for map in maps.get_rooms().iter() {
                let players = lifeforms.in_room(map, LifeformType::Player);
                let monsters = lifeforms.in_room(map, LifeformType::Monster);
                
                if players.is_some() && monsters.is_some() {
                    let events = self.get_all_monster_actions(
                        monsters.unwrap(), 
                        players.unwrap(), 
                        &lifeforms
                    );
                    
                    for event in events {
                        actions.single_write(event); 
                    }
                }
            }
        }
    }
}
