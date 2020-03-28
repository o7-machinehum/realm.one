use amethyst::{
    core:: {Time, Transform, Parent},
    ecs::{Read, System, WriteStorage, Join, Entities, Entity},
    renderer::SpriteRender
};


use crate::{
    components::{MeleeAnimation},
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
        WriteStorage<'s, Parent>,
        Entities<'s>, 
        Read<'s, Time>,
        Read<'s, SpritesContainer>,
    );

    fn run(&mut self, (mut transforms, mut sprite_renders, mut anims, mut parent, entities, time, s): Self::SystemData) {
        for item in self.delete_list.pop() {
            anims.remove(item);  // Remove the animation component
        }

        for (anim, e) in (&mut anims, &entities).join() {
            anim.update(time.delta_seconds());

            match anim.get_seq() {
                Some(new_spr) => {
                    if let Some(sprite_render) = sprite_renders.get_mut(e) {
                        sprite_render.sprite_number = new_spr;
                    }
                }
                None => ()
            };

            if anim.delete() {
                // Delete the sword entity
                entities.delete(anim.sword.unwrap().clone());
                self.delete_list.push(e.clone());
            }

            if anim.sword.is_none() {
                anim.sword = Some(entities
                .build_entity()
                .with(s.sprites[anim.sword_spr].clone(), &mut sprite_renders)
                .with(anim.sword_pos.clone(), &mut transforms) 
                .with(Parent::new(e), &mut parent) 
                .build()); 
            }
        }
    }
}
