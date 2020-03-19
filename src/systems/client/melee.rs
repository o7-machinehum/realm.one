use amethyst::{
    core:: {Time, Transform},
    ecs::{Read, System, WriteStorage, ReadStorage, Join, Entities, Entity},
    renderer::SpriteRender
};

use crate::{
    components::{MeleeAnimation, PlayerComponent},
    resources::{SpritesContainer},
};

pub struct MeleeAnimationSystem {
    delete_list : Vec::<Entity>,
    ent: Option<Entity>,
    com: Option<MeleeAnimation>,
}

impl MeleeAnimationSystem {
    pub fn new() -> Self {
        Self {
            delete_list: Vec::<Entity>::new(),
            ent: None,
            com: None,
        }
    }
}

impl<'s> System<'s> for MeleeAnimationSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, MeleeAnimation>,
        Entities<'s>, 
        Read<'s, Time>,
        Read<'s, SpritesContainer>,
    );

    fn run(&mut self, (mut transforms, mut sprite_renders, mut anims, entities, time, s): Self::SystemData) {
        for item in self.delete_list.pop() {
            anims.remove(item);
        }
        
        for (sprite_render, anim, e) in (&mut sprite_renders, &mut anims, &entities).join() {
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

            if anim.draw_sword == true {
                entities
                    .build_entity()
                    .with(s.sprites[anim.sword].clone(), &mut sprite_renders)
                    .with(anim.sword_pos, &mut transforms) 
                    .build();
            }
        }
    }
}
