use amethyst::{
    derive::SystemDesc,
    ecs::{Write, Read, System, SystemData},
};
use log::info;

use crate::{
    network::{Pack, Cmd},
    components::{Action, get_outfit, PlayerComponent},
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

    fn run(&mut self, (mut io, mut pl, maps): Self::SystemData) {
        for element in io.i.pop() {
            match &element.cmd {
                Cmd::Action(act) => {
                    info!("Action from Address: {:?}, Action: {:?}", element.ip(), element.cmd);
                    let mut acting_player = pl.get_from_ip(element.ip().unwrap()).unwrap(); 
                    let packs_players = self.act(acting_player, act, &maps);
                    
                    for pack in packs_players.0 {
                        info!("{:?}", pack);
                        io.o.push(pack) 
                    }
                    
                    for player in packs_players.1 {
                        info!("{:?}", player);
                        pl.replace(player); 
                    }
                },
                Cmd::RemovePlayer(ip) => {
                    pl.remove_with_ip(*ip); 
                },
                _ => (io.i.push(element)), 
            }
        }
    }
}

impl PlayerManSystem {
    fn act(&mut self, mut player: PlayerComponent, act: &Action, maps: &MapList) -> (Vec<Pack>, Vec<PlayerComponent>) {
        let mut out = Vec::<Pack>::new();
        let mut players = Vec::<PlayerComponent>::new();

        match act {
            Action::Move(dir) => {
                player.orientation = dir.clone();
                info!("Checking to see if walk is allowed"); 
                if maps.get(&player.room).unwrap().allowed_move(&player.trans(), &player.orientation) {
                    info!("Player Walking"); 
                    player.walk();
                    players.push(player.clone());
                    out.push(Pack::new(Cmd::UpdatePlayer(player), 0, None));
                }
            },
            Action::ChangeOutfit(skin) => {
                player.skin = get_outfit(&skin);
                //TODO: Make sure skin in legal!
                players.push(player.clone());
                out.push(Pack::new(Cmd::UpdatePlayer(player), 0, None));
            },
            _ => (), 
        };
        (out, players)
    }
}
