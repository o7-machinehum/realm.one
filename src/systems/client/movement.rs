use amethyst::{
    core:: {Transform, Time},
    ecs::{Read, System, WriteStorage, ReadStorage, Join, Entities, Entity},
    renderer::SpriteRender
};

use log::info;
use crate::{
    components::{WalkAnimation, PlayerComponent, Move},
};

pub struct MoveSystem {
    delete_list : Vec::<Entity>,
}

impl MoveSystem {
    pub fn new() -> Self {
        Self {
            delete_list: Vec::<Entity>::new(),
        }
    }
}

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Move>,
        WriteStorage<'s, Transform>,
        Entities<'s>, 
        Read<'s, Time>,
    );

    fn run(&mut self, (mut moves, mut transforms, entities, time): Self::SystemData) {
        for item in self.delete_list.pop() {
            moves.remove(item);
        }
        
        for (e, move_, tr) in (&entities, &mut moves, &mut transforms).join() {
            move_.update(time.delta_seconds());

            tr.set_translation(move_.pos());
        
            if move_.delete() {
                tr.set_translation(move_.end());
                self.delete_list.push(e.clone());
            }
        }
    }
}
