use amethyst::{
    core::{SystemDesc, Transform, bundle::SystemBundle},
    derive::SystemDesc,
    ecs::{Read, Write, World, Entities, Entity, System, SystemData, WriteStorage, DispatcherBuilder},
    shrev::{EventChannel, ReaderId},
    renderer::{SpriteRender},
    winit::{WindowEvent, Event},
    Result, 
};


use std::time::Instant;
use log::info;

use crate::{
    components::{Action, Skins},
    network::{Pack, Cmd},
    resources::{IO, SpritesContainer},
    constants,
    mech::get_letter,
};

pub struct ChatSystemBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for ChatSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            ChatSystemDesc::default().build(world),
            "chat_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct ChatSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, ChatSystem> for ChatSystemDesc {
    fn build(self, world: &mut World) -> ChatSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <ChatSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let reader = world
            .fetch_mut::<EventChannel<Event>>()
            .register_reader();
        ChatSystem::new(reader)
    }
}

#[derive(SystemDesc)]
pub struct ChatSystem {
    p1_input: String,
    ent_cmd: bool,
    send_cmd: bool,
    cmd: String,
    ent: Vec<Entity>,
    cursor: Transform,
    timer: Instant,
    event_reader: ReaderId<Event>,
    key: Option<char>,
}

impl ChatSystem {
    pub fn new (reader: ReaderId<Event>) -> Self {
        let mut tr = Transform::default();
        tr.move_up(760.0);
        tr.move_right(10.0);
        Self {
            p1_input: String::new(),
            ent_cmd: false,
            send_cmd: false,
            cmd: String::new(),
            ent: Vec::<Entity>::new(),
            cursor: tr, 
            timer: Instant::now(),
            event_reader: reader,
            key: None,
         }
    }

    fn home_cursor(&mut self) {
        self.cursor.set_translation_xyz(10.0, 760.0, 2.0);
    }
}

impl<'s> System<'s> for ChatSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, IO>,
        Read<'s, EventChannel<Event>>,
        Read<'s, SpritesContainer>,
    );
 
    fn run(&mut self, (entities, mut transforms, mut sprite_renders, mut io, events, s): Self::SystemData) {
        let now = Instant::now();
        if now.duration_since(self.timer).as_millis() >= constants::TYPING_DELAY_MS {
            for event in events.read(&mut self.event_reader) {
                if let Event::WindowEvent { ref event, .. } = *event {
                    self.key = match event { 
                        WindowEvent::ReceivedCharacter(ch) => Some(*ch), 
                        _ => None,
                    }
                }
            }
            self.timer = Instant::now();
        }

        match self.key {
            Some(k) => {
                // info!("{}", k as u8);
                match k as u8 {
                    b'/' => self.ent_cmd = true,   //
                    13   => { //Enter
                        self.ent_cmd = false;
                        self.send_cmd = true;
                    },
                    8    => {                      // Backspace 
                        self.cmd.pop(); 
                        match self.ent.pop() {
                            Some(e) => {
                                entities.delete(e).expect("Fail");
                                self.cursor.move_left(8.0);
                            }
                           None => ()
                        }
                    },
                    _ => {
                        if self.ent_cmd == true {
                            self.cmd.push(k); 
                            let e = entities
                                .build_entity()
                                .with(get_letter(k as u8, &s.text), &mut sprite_renders)
                                .with(self.cursor.clone(), &mut transforms) 
                                .build();
                            self.cursor.move_right(8.0);
                            self.ent.push(e);
                        }
                    },
                }
                self.key = None;
                    
            },
            None => ()
        }

        if self.send_cmd {
            info!("{}", self.cmd);
            
            for e in self.ent.drain(..) {
                entities.delete(e).expect("Fail");
            }
            
            let act = if self.cmd.contains("cmd") {
                parse_command(&mut self.cmd)
            }
            else {
                None
            };
            
            match act {
                Some(a) => io.o.push(Pack::new(Cmd::Action(a), 0, None)),
                None => (),
            }
            
            self.home_cursor();
            self.cmd.clear(); 
            self.send_cmd = false;
        } 
    }
}

fn parse_command(cmd: &mut String) -> Option<Action>{
    let mut iter = cmd.split_whitespace();
    
    //"cmd"
    match iter.next() {
       Some(_thing) => (),
       None => (),
    }

    // This amount of matched is too many matches.
    // It looks like a fucking spaceship 
    match iter.next() {
        Some(arg1) => {
            match arg1 {
                "outfit" => {
                    info!("Change Outfit");
                    match iter.next() {
                        Some(arg2) => {
                            info!("{}", arg2);
                            match arg2 {
                                "nude" => Some(Action::ChangeOutfit(Skins::Nude)),
                                "male"   => Some(Action::ChangeOutfit(Skins::Male)),
                                "female" => Some(Action::ChangeOutfit(Skins::Female)),
                                "skeleton" => Some(Action::ChangeOutfit(Skins::Skeleton)),
                                "slime" => Some(Action::ChangeOutfit(Skins::Slime)),
                                "bat" => Some(Action::ChangeOutfit(Skins::Bat)),
                                "ghost" => Some(Action::ChangeOutfit(Skins::Ghost)),
                                "spider" => Some(Action::ChangeOutfit(Skins::Spider)),
                                _ => None,
                            }
                        },
                        None => None 
                    }
                }
                _ => None,
            }
        },
        None => None 
    }
}
