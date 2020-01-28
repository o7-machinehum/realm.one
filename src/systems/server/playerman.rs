use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Write, Read, System, SystemData, World},
};

use crate::network::{Pack, IO, Cmd};
use crate::components::{PlayerList, Action};
use crate::map::{MapList, Room};
use log::info;

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
                            player.action(act.clone());     // Do the thing
                        }
                    }
                    // io.o.push(ready_player_one(element.ip()));
                },
                _ => (io.i.push(element)), 
            }
        }
        
        // for player in &mut players.list {     // For all the players in game
        //     if player.modified {              // If one has been modified
        //         self.new_players.push(Pack::new(Cmd::InsertPlayers(Vec::new()), 0)); // Send out the new pack
        //         info!("Inserting Player {:?}", self.new_players); 
        //         player.modified = false; 
        //     }
        // }
    } 
}
