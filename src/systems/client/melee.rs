use amethyst::{
    core:: {Time},
    ecs::{Read, System, WriteStorage, ReadStorage, Join, Entities, Entity},
    renderer::SpriteRender
};

use crate::{
    components::{MeleeAnimation, PlayerComponent},
};

pub struct MeleeAnimationSystem {
    delete_list : Vec::<Entity>,
}

impl MeleeAnimationSystem {
    pub fn new() -> Self {
        Self {
            delete_list: Vec::<Entity>::new(),
        }
    }
}

impl<'s> System<'s> for MeleeAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, MeleeAnimation>,
        ReadStorage<'s, PlayerComponent>,
        Entities<'s>, 
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut anims, players, entities, time): Self::SystemData) {
        for item in self.delete_list.pop() {
            anims.remove(item);
        }
        
        for (e, sprite_render, anim, player) in (&entities, &mut sprite_renders, &mut anims, &players).join() {
            anim.update(time.delta_seconds());

            match anim.get_seq() {
                Some(new_spr) => {
                    sprite_render.sprite_number = new_spr;
                }
                None => ()
            };

            if anim.delete() {
                self.delete_list.push(e.clone());
            }
        }
    }
}
