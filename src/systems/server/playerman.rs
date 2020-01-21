use amethyst::{
    core::{SystemDesc},
    derive::SystemDesc,
    ecs::{Entities, Join, Write, System, SystemData, World, WriteStorage},
    network::*,
};

use std::{
    fs::File,
};
use std::io::Read;

use crate::network;
use log::info;
use crate::network::{Pack, Cmd, IO};
use crate::components::PlayerList;

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

    fn run(&mut self, (mut io, mut players): Self::SystemData) {
        // for player in &mut players.list {     // For all the players in game
        //     if player.modified {              // If one has been modified
        //         self.new_players.push(Pack::new(Cmd::InsertPlayers(Vec::new()), 0)); // Send out the new pack
        //         info!("Inserting Player {:?}", self.new_players); 
        //         player.modified = false; 
        //     }
        // }
    } 
}
