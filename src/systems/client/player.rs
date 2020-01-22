use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, Write, Entities, Entity, System, SystemData, World, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::renderer::SpriteRender;

use std::time::Instant;
use log::info;

use crate::components::{PlayerComponent, Action};
use crate::key_bindings::{MovementBindingTypes, AxisBinding};
use crate::map::{Room, Adj, SpritesContainer};
use crate::network::{Pack, IO, Cmd};
use crate::constants;


#[derive(SystemDesc)]
pub struct PlayerSystem{ 
    pub p1: Option<Entity>,
}

impl<'s> System<'s> for PlayerSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, PlayerComponent>,
        WriteStorage<'s, SpriteRender>,
        Write<'s, IO>,
        Write<'s, Room>,
        Entities<'s>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, SpritesContainer>,
    );
 
    fn run(&mut self, (mut transforms, mut players, mut sprite_renders, mut io, room, entities, input, s): Self::SystemData) {
        for element in io.i.pop() {
            match &element.cmd {
                Cmd::InsertPlayer(p1) =>  {
                    info!("Inserting Player"); 
                    let player = PlayerComponent::new(p1.clone(), &s.sprites);
                    self.p1 = Some(entities
                        .build_entity()
                        .with(player.trans(), &mut transforms)
                        .with(player.get_orientated().clone(), &mut sprite_renders)
                        .with(player, &mut players) 
                        .build());
                    },
                _ => io.i.push(element), 
            }
        }
        
        match self.p1 {
            Some(p1) => {
                let now = Instant::now();
                let mut player = players.get_mut(p1).unwrap();

                if now.duration_since(player.last_movement_instant).as_millis() >= constants::MOVEMENT_DELAY_MS {
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
                    player.last_movement_instant = now.clone();
                    sprite_renders.insert(p1, player.get_orientated()).expect("Failed to insert orientated player!");

                    let adj: Adj = room.get_adj(tr);
                    if room.allowed_move(tr, horizontal, vertical, adj){
                        player.walk(&horizontal, &vertical);
                        tr.set_translation_xyz(player.x(), player.y(), player.z()); 
                        io.o.push(Pack::new(Cmd::Action(Action::Move(player.orientation.clone())), 0, None));
                    }
                }
            },
            None => ()
        }
    }
}
