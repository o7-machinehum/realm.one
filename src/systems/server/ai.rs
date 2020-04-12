use amethyst::{
    core::{SystemDesc, bundle::SystemBundle},
    derive::SystemDesc,
    ecs::{Write, World, Read, System, SystemData, DispatcherBuilder},
    shrev::{EventChannel},
    Result, 
};

use log::info;
use crate::{
    resources::{LifeformList},
    systems::server::{LifeformEvent},
};

/// Events that pertain to the Ai System

#[derive(SystemDesc)]
pub struct AiSystem; 

pub struct AiSystemBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for AiSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            AiSystemDesc::default().build(world),
            "auth_system",
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
        AiSystem
    }
}

impl<'a> System<'a> for AiSystem {
    type SystemData = (
        Write<'a, EventChannel<LifeformEvent>>,
        Read <'a, LifeformList>,
    );

    fn run(&mut self, (mut actions, lifeforms): Self::SystemData) {
         
    }
}
