use amethyst::{
    derive::SystemDesc,
    ecs::{Write, Read, System, SystemData},
};
use log::info;

use crate::{
    network::{Pack, Cmd},
    components::{Action, get_outfit, LifeformComponent},
    resources::{LifeformList, IO, MapList},
};

#[derive(SystemDesc)]
pub struct LifeformAiSystem {
    init: bool, // Have you added all the monsters to the list?
};

impl LifeformAiSystem {
    pub fn new() -> Self{
        Self {
            init: false,
        }
    }
}

/// This system should determine the most reasonable thing for each monster to do
/// Each monster should have they're own "unit time" so it's not happeneing all at once
/// for now they can share one timer
impl<'a> System<'a> for LifeformManSystem {
    type SystemData = (
        Write <'a, IO>,
        Write<'a, LifeformList>,
        Read <'a, MapList>,
        Write <'a, LifeformUID>,
    );

    fn run(&mut self, (mut io, mut lifeforms, maps, mut uid): Self::SystemData) {
        if !self.init {
            // The load up all the monsters
            // Below is just a test
            lifeforms.add(LifeformComponent::new_monster(uid.add(), "Mr. Dood".to_string(), (16.0, 16.0) // ...) 
            init = true;
        }
        
        io.push()
    }
}
