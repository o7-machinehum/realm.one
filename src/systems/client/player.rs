use amethyst::{
    core::{Transform, Parent},
    derive::SystemDesc,
    ecs::{Read, Write, Entities, Entity, System, SystemData, WriteStorage},
    input::InputHandler,
    renderer::SpriteRender
};
use nalgebra::base::Vector3;

use std::time::Instant;
use log::info;

use crate::{
    components::{PlayerComponent, Action},
    key_bindings::{MovementBindingTypes, AxisBinding},
    map::{Room},
    network::{Pack, Cmd},
    resources::{IO, SpritesContainer},
    constants,
    mech::get_letter,
};


#[derive(SystemDesc)]
pub struct PlayerSystem { 
    pub p1: Option<Entity>,
    pub timer: Option<Instant>,
    pub p1_name: String,
}

impl<'s> System<'s> for PlayerSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PlayerComponent>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, IO>,
        Write<'s, Room>,
        Entities<'s>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, SpritesContainer>,
    );
 
    fn run(&mut self, (mut transforms, mut players, mut parents, mut sprite_renders, mut io, room, entities, input, s): Self::SystemData) {
        for element in io.i.pop() {
            match &element.cmd {
                Cmd::InsertPlayer(play) =>  {
                    let player = PlayerComponent::new(play.clone());
                    let e = Some(entities
                        .build_entity()
                        .with(player.trans(), &mut transforms)
                        .with(player.get_orientated(&s.sprites), &mut sprite_renders)
                        .with(player, &mut players) 
                        .build());
                    
                    // Write the players name
                    let mut letter_trans = play.trans();
                    letter_trans.move_up(10.0);
                    letter_trans.set_scale(Vector3::new(0.8, 0.8, 0.8));
                    for bytes in play.name.bytes() {
                        entities
                            .build_entity()
                            .with(get_letter(bytes, &s.text), &mut sprite_renders)
                            .with(letter_trans.clone(), &mut transforms) 
                            .with(Parent::new(e.unwrap()), &mut parents)
                            .build();
                        letter_trans.move_right(15.0);
                    }
                    if play.name == self.p1_name { 
                        info!("Inserting Player 1"); 
                        self.p1 = e;
                        self.timer = Some(Instant::now());
                    }
                },
                _ => io.i.push(element), 
            }
        }
        
        match self.p1 {
            Some(p1) => {
                let now = Instant::now();
                let player = players.get_mut(p1).unwrap();

                if now.duration_since(self.timer.unwrap()).as_millis() >= constants::MOVEMENT_DELAY_MS {
                    let horizontal = input
                        .axis_value(&AxisBinding::Horizontal)
                        .unwrap_or(0.0);
                    let vertical = input
                        .axis_value(&AxisBinding::Vertical)
                        .unwrap_or(0.0);
                    
                    if horizontal == 0. && vertical == 0. {
                        return;
                    }
                    
                    let tr = transforms.get_mut(p1).unwrap(); 

                    player.update_orientation(&horizontal, &vertical);
                    self.timer = Some(now.clone());
                    sprite_renders.insert(p1, player.get_orientated(&s.sprites)).expect("Failed to insert orientated player!");

                    if room.allowed_move(tr, &player.orientation) {
                        player.walk(); // Walk one step in forward direction
                        tr.set_translation_xyz(player.x(), player.y(), player.z()); 
                        io.o.push(Pack::new(Cmd::Action(Action::Move(player.orientation.clone())), 0, None));
                    }
                }
            },
            None => ()
        }
    }
}
