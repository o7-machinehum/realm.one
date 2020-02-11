use amethyst::{
    core:: {Transform, Time},
    ecs::{Read, System, WriteStorage, ReadStorage, Join, Entities, Entity},
    renderer::SpriteRender
};

use log::info;
use crate::{
    components::{SimpleAnimation, PlayerComponent},
};

pub struct SimpleAnimationSystem {
    delete_list : Vec::<Entity>,
}

impl SimpleAnimationSystem {
    pub fn new() -> Self {
        Self {
            delete_list: Vec::<Entity>::new(),
        }
    }
}

impl<'s> System<'s> for SimpleAnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, SimpleAnimation>,
        ReadStorage<'s, PlayerComponent>,
        WriteStorage<'s, Transform>,
        Entities<'s>, 
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut anims, players, mut transforms, entities, time): Self::SystemData) {
        for item in self.delete_list.pop() {
            anims.remove(item);
        }
        
        for (e, sprite_render, anim, player, tr) in (&entities, &mut sprite_renders, &mut anims, &players, &mut transforms).join() {
            anim.update(time.delta_seconds());

            match anim.get_seq() {
                Some(new_footing) => {
                    sprite_render.sprite_number = (player.get_dir() as i32 + new_footing) as usize;
                }
                None => ()
            };

            // tr.set_translation(anim.pos());
            tr.set_translation(anim.pos());
        
            if anim.delete() {
                // Clean up to a reasonable number
                tr.set_translation(*player.trans().translation());
                self.delete_list.push(e.clone());
            }
        }
    }
}
