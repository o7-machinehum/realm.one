use amethyst::{
    core::{SystemDesc, Transform, bundle::SystemBundle},
    derive::SystemDesc,
    ecs::{Write, Read, World, System, SystemData, DispatcherBuilder},
    shrev::{EventChannel, ReaderId},
    Result, 
};
use log::info;

use crate::{
    network::{Pack, Cmd, Dest},
    components::{Action, get_outfit, LifeformComponent},
    resources::{LifeformList, IO, MapList},
};
use std::net::{SocketAddr};

#[derive(Debug)]
pub enum LifeformEvent {
    RemovePlayer(u64),
    Action(Action, SocketAddr),
}

/// Lifeform manager system.
#[derive(SystemDesc)]
pub struct LifeformSystem {
    event_reader: ReaderId<LifeformEvent>,
}

pub struct LifeformSystemBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for LifeformSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            LifeformSystemDesc::default().build(world),
            "lifeform_man_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct LifeformSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, LifeformSystem> for LifeformSystemDesc {
    fn build(self, world: &mut World) -> LifeformSystem {
        <LifeformSystem as System<'_>>::SystemData::setup(world);
        let event_reader = world
            .fetch_mut::<EventChannel<LifeformEvent>>()
            .register_reader();
        LifeformSystem{ event_reader }
    }
}

impl<'a> System<'a> for LifeformSystem {
    type SystemData = (
        Write<'a, EventChannel<Pack>>,
        Read<'a, EventChannel<LifeformEvent>>,
        Write<'a, LifeformList>,
        Read <'a, MapList>,
    );

    fn run(&mut self, (mut cmd_out, events, mut pl, maps): Self::SystemData) {
        for event in events.read(&mut self.event_reader) {
           match &event {
                LifeformEvent::Action(act, ip) => {
                    info!("Action from Address: {:?}, Action: {:?}", ip, act);
                    let acting_player = pl.get_from_ip(*ip).unwrap(); 
                    info!("player gotten from IP is: {:?}", acting_player);
                    let packs_players = self.act(acting_player, act, &maps, &pl);
                    
                    // If packs come out of the action
                    for pack in packs_players.0 {
                        info!("{:?}", pack);
                        cmd_out.single_write(pack)
                    }

                    // If a player needs to be replacd  
                    for player in packs_players.1 {
                        info!("{:?}", player);
                        pl.replace(player); 
                    }
                },
                LifeformEvent::RemovePlayer(uid) => pl.remove_with_id(*uid), 
            }
        }
    }
}

impl LifeformSystem {
    fn act(&mut self, 
           mut player: LifeformComponent, 
           act: &Action, 
           maps: &MapList, 
           pl: &LifeformList) 
           -> (Vec<Pack>, Vec<LifeformComponent>) 
        {
        let mut out = Vec::<Pack>::new();
        let mut players = Vec::<LifeformComponent>::new();

        match act {
            Action::Move(dir) => {
                player.orientation = dir.clone();
                info!("Checking to see if walk is allowed"); 
                if maps.get(&player.room).unwrap().allowed_move(&player.trans(), &player.orientation) {
                    info!("Player Walking"); 
                    player.walk();
                    players.push(player.clone());
                    let rm = player.room.clone();
                    out.push(Pack::new(Cmd::UpdatePlayer(player), Dest::Room(rm)));
                }
            },
            
            Action::ChangeOutfit(skin) => {
                player.skin = get_outfit(&skin);
                //TODO: Make sure skin in legal!
                players.push(player.clone());
                let rm = player.room.clone();
                out.push(Pack::new(Cmd::UpdatePlayer(player), Dest::Room(rm)));
            },

            Action::Melee => {
                let victom = pl.get_from_transform(player.in_front()); // Anyone in front of the player???
                info!("Swing!"); 
                match victom{
                    Some(mut victom) => {
                        info!("Direct Hit!");
                        victom.hp(-10.0); // Oh shit
                        players.push(victom.clone());
                        let rm = player.room.clone();
                        out.push(Pack::new(Cmd::UpdatePlayer(victom), Dest::Room(rm)));
                    },
                    None => info!("And a miss!"), 
                }
            },
            
            Action::Rotate(dir) => {
                player.orientation = dir.clone();
                players.push(player.clone());
                let rm = player.room.clone();
                out.push(Pack::new(Cmd::UpdatePlayer(player), Dest::Room(rm)));
            },
            _ => (), 
        };
        (out, players)
    }
}
