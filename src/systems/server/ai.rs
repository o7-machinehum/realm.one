use amethyst::{
    core::{SystemDesc, bundle::SystemBundle},
    derive::SystemDesc,
    ecs::{Write, World, Read, System, SystemData, DispatcherBuilder},
    shrev::{EventChannel},
    Result, 
};

use log::info;
use std::time::Instant;

use crate::{
    resources::{LifeformList},
    components::{LifeformType, Action, Orientation, LifeformComponent},
    systems::server::{LifeformEvent},
};

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
    fn monster_action(
        &self, 
        monster: &LifeformComponent, 
        players: &Vec<LifeformComponent>,
    ) -> Option<LifeformEvent>
    {
        Some(LifeformEvent::Action(
            Action::Move(
                Orientation::North),
                monster.clone() 
        ))
    }
}

impl<'a> System<'a> for AiSystem {
    type SystemData = (
        Write<'a, EventChannel<LifeformEvent>>,
        Read <'a, LifeformList>,
    );

    fn run(&mut self, (mut actions, lifeforms): Self::SystemData) {
        let now = Instant::now();

        if now.duration_since(self.timer).as_millis() >= 5000 {
            self.timer = now.clone();
            for lifeform in &lifeforms.list  {
                if let Some(lf) = lifeform {
                    if lf.kind == LifeformType::Monster {
                        //if let Some(act) = self.monster_action(lf){
                        //    actions.single_write(act); 
                        //}
                    }
                }
            }
        }

        //thread::sleep(time::Duration::from_millis(500));  
        
    }
}
