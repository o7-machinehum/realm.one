use amethyst::{
    derive::SystemDesc,
    ecs::{Write, Read, System, SystemData},
};
use log::info;

use crate::{
    network::{Pack, Cmd},
    components::{Action},
    resources::{PlayerList, IO, MapList},
};

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct PlayerManSystem;

impl<'a> System<'a> for PlayerManSystem {
    type SystemData = (
        Write <'a, IO>,
        Write<'a, PlayerList>,
        Read <'a, MapList>,
    );

    fn run(&mut self, (mut io, mut players, maps): Self::SystemData) {
        for element in io.i.pop() {
            match &element.cmd {
                Cmd::Action(act) => {
                    info!("Action from Address: {:?}, Action: {:?}", element.ip(), element.cmd);
                    for mut player in &mut players.list {
                        if player.ip == element.ip().unwrap() {
                            // Get room the player is in
                            let mut k : usize = 0;
                            for x in 0..maps.list.len() {
                                if maps.list[x].name == player.room {
                                    k = x;
                                    break
                                }
                            };

                            let pk = match act {
                                Action::Move(dir) => {
                                    player.orientation = dir.clone();
                                    info!("Checking to see if walk is allowed"); 
                                    if maps.list[k].allowed_move(&player.trans(), &player.orientation) {
                                        info!("Player Walking"); 
                                        player.walk();
                                        Some(Pack::new(Cmd::UpdatePlayer(player.clone()), 0, None))
                                    }
                                    else {
                                        None
                                    }
                                },
                                _ => None, 
                            };

                            info!("{:?}", pk);
                            
                            match pk {
                                Some(pk) => io.o.push(pk),
                                None => (),
                            }
                        }
                    }
                },
                Cmd::RemovePlayer(ip) => {
                    let mut p = 0;
                    for i in 0..players.list.len() {
                        if players.list[i].ip == *ip {
                            info!("Removing player: {}", *ip); 
                            p = i;
                            break; 
                        }
                    }

                    players.list.remove(p);
                },
                _ => (io.i.push(element)), 
            }
        }
    }
}
