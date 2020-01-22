use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Write, System, SystemData, World},
};

use crate::network::{Pack, IO, Cmd};
use crate::components::{PlayerList, Action};
use log::info;

/// A simple system that receives a ton of network events.
#[derive(SystemDesc)]
pub struct PlayerManSystem {
    pub new_players: Vec<Pack>, 
}

impl<'a> System<'a> for PlayerManSystem {
    type SystemData = (
        Write <'a, IO>,
        Write<'a, PlayerList>,
    );

    fn run(&mut self, (mut io, players): Self::SystemData) {
        for element in io.i.pop() {
            match &element.cmd {
                Cmd::Action(act) => {
                    info!("Action from Address: {:?}, Action: {:?}", element.ip(), element.cmd);
                    // io.o.push(insert_map(packet.to_string(), element.ip())); 
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
