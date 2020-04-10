use amethyst::{
    core::{Parent, Transform},
    derive::SystemDesc,
    ecs::{Entities, Entity, Join, Read, System, SystemData, Write, WriteStorage},
    renderer::resources::Tint,
    renderer::SpriteRender,
};

use log::info;
use std::time::Instant;

use crate::{
    components::{Action, LifeformComponent, MeleeAnimation, Move, WalkAnimation},
    constants,
    map::Room,
    mech::get_letter,
    network::{Cmd, Dest, Pack},
    resources::{Input, Inputs, SpritesContainer, IO},
};

#[derive(SystemDesc)]
pub struct PlayerSystem {
    p1: Option<Entity>,
    timer: Option<Instant>,
    p1_name: String,
}

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Move>,
        WriteStorage<'s, WalkAnimation>,
        WriteStorage<'s, MeleeAnimation>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, LifeformComponent>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        Write<'s, IO>,
        Write<'s, Room>,
        Entities<'s>,
        Write<'s, Input>,
        Read<'s, SpritesContainer>,
    );

    fn run(
        &mut self,
        (
            mut moves,
            mut walk,
            mut swing,
            mut transforms,
            mut players,
            mut parents,
            mut sprite_renders,
            mut tints,
            mut io,
            room,
            entities,
            mut input,
            s,
        ): Self::SystemData,
    ) {
        for element in io.i.pop() {
            match &element.cmd {
                Cmd::InsertPlayer(play) => {
                    let e = Some(
                        entities
                            .build_entity()
                            .with(play.trans(), &mut transforms)
                            .with(play.get_orientated(&s.sprites), &mut sprite_renders)
                            .with(Tint(play.tint()), &mut tints)
                            .with(play.clone(), &mut players)
                            .build(),
                    );
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
                }
                Cmd::InsertPlayer1(play) => {
                    let e = Some(
                        entities
                            .build_entity()
                            .with(play.trans(), &mut transforms)
                            .with(play.get_orientated(&s.sprites), &mut sprite_renders)
                            .with(Tint(play.tint()), &mut tints)
                            .with(play.clone(), &mut players)
                            .build(),
                    );
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

                    if self.p1.is_none() {
                        info!("Inserting Player 1");
                        self.p1 = e;
                        self.timer = Some(Instant::now());
                    }
                }
                _ => io.i.push(element),
            }
        }
        if self.p1.is_some() {
            let now = Instant::now();
            let p1 = self.p1.unwrap();
            if now.duration_since(self.timer.unwrap()).as_millis() >= constants::MOVEMENT_DELAY_MS {
                self.timer = Some(now.clone());
                let inp = input.get(); // Get the move
                if inp.is_some() {
                    match inp.unwrap() {
                        Inputs::Move(dir) => {
                            // Get player and transform component of yourself
                            let adj_player_tr = {
                                let player = players.get_mut(p1).unwrap(); // Get yourself
                                let spr = sprite_renders.get_mut(p1).unwrap(); // Get sprite
                                if player.update_orientation(dir) {
                                    // Update self
                                    spr.sprite_number = player.get_dir(); // Change sprite
                                    io.o.push(Pack::new(
                                        Cmd::Action(Action::Rotate(player.orientation.clone())),
                                        Dest::All,
                                    ));
                                }
                                player.in_front() // Get transform of in front
                            };

                            let mut adj_player: Option<LifeformComponent> = None;
                            for (transform, p) in (&mut transforms, &mut players).join() {
                                if *transform.translation() == *adj_player_tr.translation() {
                                    // There's someone in the way!
                                    adj_player = Some(p.clone());
                                }
                            }

                            let player = players.get_mut(p1).unwrap();
                            if room.allowed_move(&player.trans(), &player.orientation)
                                && !adj_player.is_some()
                            {
                                let tr = transforms.get_mut(p1).unwrap();
                                player.walk(); // Walk one step in forward direction

                                let mv = Move::new(
                                    *tr.translation(),
                                    *player.trans().translation(),
                                    (constants::MOVEMENT_DELAY_MS as f32) / 1000.0,
                                );

                                walk.insert(
                                    p1,
                                    WalkAnimation::new(
                                        (constants::MOVEMENT_DELAY_MS as f32) / 1000.0,
                                    ),
                                );
                                moves.insert(p1, mv);

                                io.o.push(Pack::new(
                                    Cmd::Action(Action::Move(player.orientation.clone())),
                                    Dest::All,
                                ));
                            }
                        }
                        Inputs::Melee => {
                            info!("Punch");
                            swing.insert(p1, MeleeAnimation::new(players.get_mut(p1).unwrap()));
                            io.o.push(Pack::new(Cmd::Action(Action::Melee), Dest::All));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

impl PlayerSystem {
    pub fn new(name: String) -> Self {
        Self {
            p1: None,
            timer: None,
            p1_name: name,
        }
    }
}
