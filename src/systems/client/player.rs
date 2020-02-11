use amethyst::{
    core::{Transform, Parent},
    derive::SystemDesc,
    ecs::{Read, Write, Entities, Entity, System, SystemData, WriteStorage, Join},
    input::InputHandler,
    renderer::SpriteRender, 
    renderer::resources::Tint,
    renderer::palette::rgb::Srgba,
};


use std::time::Instant;
use log::info;

use crate::{
    components::{PlayerComponent, Action, SimpleAnimation},
    key_bindings::{MovementBindingTypes, AxisBinding, ActionBinding},
    map::{Room},
    network::{Pack, Cmd},
    resources::{IO, SpritesContainer},
    constants,
    mech::get_letter,
};


#[derive(SystemDesc)]
pub struct PlayerSystem { 
    p1: Option<Entity>,
    timer: Option<Instant>,
    p1_name: String,
    horizontal: f32,
    vertical: f32,
    melee: bool,
}

impl PlayerSystem {
    pub fn new(name: String) -> Self {
        Self {
            p1: None,
            timer: None, 
            p1_name: name,
            horizontal: 0.0,
            vertical: 0.0,
            melee: false,
        }
    }

    fn get_input<'s>(&mut self, input: Read<'s, InputHandler<MovementBindingTypes>>) {
        match input.axis_value(&AxisBinding::Horizontal) {
            Some(value) => {
                self.horizontal = value; 
                // if value != 0.0 {
                //     self.horizontal = value;
                // }
            },
            None => (),
        }
        
        match input.axis_value(&AxisBinding::Vertical) {
            Some(value) => {
                self.vertical = value;
                // if value != 0.0 {
                //     self.vertical = value;
                // }
            },
            None => (),
        }

        match input.action_is_down(&ActionBinding::Melee) {
            Some(value) => {
                if value == true {
                    self.melee = true;
                }
            },
            None => (),
        }
    }
}

impl<'s> System<'s> for PlayerSystem{
    type SystemData = (
        WriteStorage<'s, SimpleAnimation>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PlayerComponent>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        Write<'s, IO>,
        Write<'s, Room>,
        Entities<'s>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, SpritesContainer>,
    );
 
    fn run(&mut self, 
         (mut anim, 
          mut transforms, 
          mut players, 
          mut parents, 
          mut sprite_renders, 
          mut tints,
          mut io, 
          room, 
          entities, 
          input, 
          s): Self::SystemData) 
    {
        for element in io.i.pop() {
            match &element.cmd {
                Cmd::InsertPlayer(play) =>  {
                    let e = Some(entities
                        .build_entity()
                        .with(play.trans(), &mut transforms)
                        .with(play.get_orientated(&s.sprites), &mut sprite_renders)
                        .with(Tint(play.tint()), &mut tints)
                        .with(play.clone(), &mut players)
                        .build());
                    
                    // Write the players name
                    let mut letter_trans = Transform::default();
                    letter_trans.move_up(10.0);
                    for bytes in play.name.bytes() {
                        entities
                            .build_entity()
                            .with(get_letter(bytes, &s.text), &mut sprite_renders)
                            .with(letter_trans.clone(), &mut transforms) 
                            .with(Parent::new(e.unwrap()), &mut parents)
                            .build();
                        letter_trans.move_right(8.0);
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
        
        if self.p1.is_some() {
            let now = Instant::now();
            let p1 = self.p1.unwrap();
            
            self.get_input(input);
                        
            if now.duration_since(self.timer.unwrap()).as_millis() >= constants::MOVEMENT_DELAY_MS {
                if self.horizontal != 0. || self.vertical != 0. {
                    // Get player and transform component of yourself
                    let adj_player_tr = { 
                        let player = players.get_mut(p1).unwrap();  // Get yourself
                        let spr = sprite_renders.get_mut(p1).unwrap();  // Get sprite 
                        if(player.update_orientation(&self.horizontal, &self.vertical)) { // Update self
                            spr.sprite_number = player.get_dir();             // Change sprite
                            io.o.push(Pack::new(Cmd::Action(Action::Rotate(player.orientation.clone())), 0, None));
                        } 
                        player.in_front()    // Get transform of in front
                    };
                    
                    let mut adj_player : Option<PlayerComponent> = None;
                    for (transform, p) in (&mut transforms, &mut players).join() {
                        if *transform.translation() == *adj_player_tr.translation(){
                            // There's someone in the way!
                            adj_player = Some(p.clone());    
                        }
                    }

                    let player = players.get_mut(p1).unwrap();
                    if room.allowed_move(&player.trans(), &player.orientation) && !adj_player.is_some() {
                        let tr = transforms.get_mut(p1).unwrap(); 
                        player.walk(); // Walk one step in forward direction

                        anim.insert(p1, SimpleAnimation::new((constants::MOVEMENT_DELAY_MS as f32) / 1000.0, 
                                                              *tr.translation(), 
                                                              *player.trans().translation()));

                        io.o.push(Pack::new(Cmd::Action(Action::Move(player.orientation.clone())), 0, None));
                    }
                    self.horizontal = 0.0;
                    self.vertical = 0.0;
                }

                if self.melee {
                    info!("Punch"); 
                    io.o.push(Pack::new(Cmd::Action(Action::Melee), 0, None));
                    self.melee = false;
                }

                self.timer = Some(now.clone());
            }
        }
    }
}
