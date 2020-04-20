use amethyst::{
    core::{SystemDesc, bundle::SystemBundle},
    derive::SystemDesc,
    ecs::{Write, Read, World, System, SystemData, DispatcherBuilder},
    shrev::{EventChannel, ReaderId},
    Result, 
};
use log::info;

use crate::{
    network::{Pack, Cmd, Dest},
    components::{Action, get_outfit, LifeformComponent, LifeformType},
    resources::{LifeformList, MapList},
};

#[derive(Debug)]
pub enum LifeformEvent {
    RemovePlayer(u64),
    Action(Action, LifeformComponent),
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
                LifeformEvent::Action(act, player_acting) => {
                    // info!("Action from Player: {:?}, Action: {:?}", player_acting, act);
                    let packs_players = self.act(player_acting.clone(), act, &maps, &pl);
                    
                    // If packs come out of the action
                    for pack in packs_players.0 {
                        // info!("{:?}", pack);
                        cmd_out.single_write(pack)
                    }

                    // If a player needs to be replacd  
                    for player in packs_players.1 {
                        // info!("{:?}", player);
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
           pl: &LifeformList,
           )-> (Vec<Pack>, Vec<LifeformComponent>) 
        {
        let mut pack_out = Vec::<Pack>::new();
        let mut players_out = Vec::<LifeformComponent>::new();

        match act {
            Action::Move(dir) => {
                player.orientation = dir.clone();
                let next_step = player.in_front();
                
                // First check for a player colision
                if let Some(players) = pl.in_room(&player.room, LifeformType::Player) {
                    for player_id in players {
                        if let Some(player) = pl.get_from_id(*player_id) {
                            if player.trans() == next_step { 
                                return (pack_out, players_out)
                            }
                        }
                    }
                }
                
                // Then check for a monster colision
                if let Some(monsters) = pl.in_room(&player.room, LifeformType::Monster) {
                    for monster_id in monsters {
                        if let Some(monster) = pl.get_from_id(*monster_id) {
                            if monster.trans() == next_step { 
                                return (pack_out, players_out)
                            }
                        }
                    }
                }
                
                // Then check for a building colision
                if maps.get(&player.room).unwrap().allowed_move(&player.trans(), &player.orientation) {
                    // info!("Player Walking"); 
                    player.walk();
                    players_out.push(player.clone());
                    let room = player.room.clone();
                    pack_out.push(Pack::new(Cmd::UpdatePlayer(player), Dest::Room(room)));
                }
            },
            
            Action::ChangeOutfit(skin) => {
                player.skin = get_outfit(&skin);
                //TODO: Make sure skin in legal!
                players_out.push(player.clone());
                let rm = player.room.clone();
                pack_out.push(Pack::new(Cmd::UpdatePlayer(player), Dest::Room(rm)));
            },

            Action::Melee => {
                let victom = pl.get_from_transform(player.in_front()); // Anyone in front of the player???
                info!("Swing!"); 
                match victom{
                    Some(mut victom) => {
                        info!("Direct Hit!");
                        victom.hp(-10.0); // Oh shit
                        players_out.push(victom.clone());
                        let rm = player.room.clone();
                        pack_out.push(Pack::new(Cmd::UpdatePlayer(victom), Dest::Room(rm)));
                    },
                    None => info!("And a miss!"), 
                }
            },
            
            Action::Rotate(dir) => {
                player.orientation = dir.clone();
                players_out.push(player.clone());
                let rm = player.room.clone();
                pack_out.push(Pack::new(Cmd::UpdatePlayer(player), Dest::Room(rm)));
            },
            _ => (), 
        };

        (pack_out, players_out)
    }
}
