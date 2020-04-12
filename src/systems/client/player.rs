use amethyst::{
    core::{Parent, Transform, SystemDesc, bundle::SystemBundle},
    derive::SystemDesc,
    ecs::{World, Entities, Entity, Join, Read, System, SystemData, Write, WriteStorage, DispatcherBuilder},
    renderer::resources::Tint,
    renderer::SpriteRender,
    shrev::{EventChannel, ReaderId},
    Result, 
};

use log::info;
use std::time::Instant;

use crate::{
    components::{Action, LifeformComponent, MeleeAnimation, Move, WalkAnimation},
    constants,
    map::Room,
    mech::get_letter,
    network::{Cmd, Dest, Pack},
    resources::{Command, CommandQueue, SpritesContainer},
};

pub enum PlayerEvent {
    InsertPlayer(LifeformComponent),
    InsertPlayer1(LifeformComponent),
}

#[derive(SystemDesc)]
pub struct PlayerSystem {
    p1: Option<Entity>,
    timer: Option<Instant>,
    event_reader: ReaderId<PlayerEvent>,
}


pub struct PlayerSystemBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for PlayerSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            PlayerSystemDesc::default().build(world),
            "player_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct PlayerSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, PlayerSystem> for PlayerSystemDesc {
    fn build(self, world: &mut World) -> PlayerSystem {
        <PlayerSystem as System<'_>>::SystemData::setup(world);
        let event_reader = world
            .fetch_mut::<EventChannel<PlayerEvent>>()
            .register_reader();
        PlayerSystem::new( event_reader )
    }
}

impl PlayerSystem {
    pub fn new(event_reader: ReaderId<PlayerEvent>) -> Self {
        Self {
            p1: None,
            timer: None,
            event_reader,
        }
    }
}

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        Write <'s, EventChannel<Pack>>,
        Read <'s, EventChannel<PlayerEvent>>,
        WriteStorage<'s, Move>,
        WriteStorage<'s, WalkAnimation>,
        WriteStorage<'s, MeleeAnimation>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, LifeformComponent>,
        WriteStorage<'s, Parent>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        Write<'s, Room>,
        Entities<'s>,
        Write<'s, CommandQueue>,
        Read<'s, SpritesContainer>,
    );

    fn run(
        &mut self,
        (
            mut cmd_out,
            events,
            mut moves,
            mut walk,
            mut swing,
            mut transforms,
            mut players,
            mut parents,
            mut sprite_renders,
            mut tints,
            room,
            entities,
            mut command_queue,
            s,
        ): Self::SystemData,
    ) {
        for event in events.read(&mut self.event_reader) {
            match &event {
                PlayerEvent::InsertPlayer(play) => {
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
                PlayerEvent::InsertPlayer1(play) => {
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
            }
        }
        if self.p1.is_some() {
            let now = Instant::now();
            let p1 = self.p1.unwrap();
            if now.duration_since(self.timer.unwrap()).as_millis() >= constants::ACTION_DELAY_MS {
                self.timer = Some(now.clone());
                let cmd = command_queue.get(); // Get the move
                if cmd.is_some() {
                    match cmd.unwrap() {
                        Command::Move(dir) => {
                            // Get player and transform component of yourself
                            let adj_player_tr = {
                                let player = players.get_mut(p1).unwrap(); // Get yourself
                                let spr = sprite_renders.get_mut(p1).unwrap(); // Get sprite
                                if player.update_orientation(dir) {
                                    // Update self
                                    spr.sprite_number = player.get_dir(); // Change sprite
                                    cmd_out.single_write(Pack::new(
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
                                    (constants::ACTION_DELAY_MS as f32) / 1000.0,
                                );

                                walk.insert(
                                    p1,
                                    WalkAnimation::new(
                                        (constants::ACTION_DELAY_MS as f32) / 1000.0,
                                    ),
                                ).expect("Could not insert walk entity!");
                                moves.insert(p1, mv).expect("Cannot insert player");

                                cmd_out.single_write(Pack::new(
                                    Cmd::Action(Action::Move(player.orientation.clone())),
                                    Dest::All,
                                ));
                            }
                        }
                        Command::Melee => {
                            info!("Punch");
                            swing.insert(p1, MeleeAnimation::new(players.get_mut(p1).unwrap()))
                                .expect("Could not insert player!");
                            cmd_out.single_write(Pack::new(Cmd::Action(Action::Melee), Dest::All));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
