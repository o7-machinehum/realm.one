use amethyst::{
    core::{SystemDesc, Transform, bundle::SystemBundle},
    derive::SystemDesc,
    ecs::{Write, World, Read, System, SystemData, DispatcherBuilder},
    shrev::{EventChannel, ReaderId},
    Result, 
};

use log::info;
use crate::{
    network::{Pack, Cmd, Dest},
    components::{LifeformComponent},
    resources::{LifeformList, IO, MapList, LifeformUID},
};

use std::net::{SocketAddr};
use std::iter::{Iterator};

/// Events that pertain to the Auth System
#[derive(Debug)]
pub enum AuthEvent {
    Connect( String, SocketAddr), 
}

#[derive(SystemDesc)]
pub struct AuthSystem {
    event_reader: ReaderId<AuthEvent>,
}


pub struct AuthSystemBundle;
impl<'a, 'b> SystemBundle<'a, 'b> for AuthSystemBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            AuthSystemDesc::default().build(world),
            "auth_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct AuthSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, AuthSystem> for AuthSystemDesc {
    fn build(self, world: &mut World) -> AuthSystem {
        <AuthSystem as System<'_>>::SystemData::setup(world);
        let event_reader = world
            .fetch_mut::<EventChannel<AuthEvent>>()
            .register_reader();
        AuthSystem{ event_reader }
    }
}

impl<'a> System<'a> for AuthSystem {
    type SystemData = (
        Write<'a, EventChannel<Pack>>,
        Write <'a, IO>,
        Read <'a, EventChannel<AuthEvent>>,
        Write <'a, LifeformList>,
        Read <'a, MapList>,
        Write <'a, LifeformUID>,
    );

    fn run(&mut self, (mut cmd_out, mut io, mut ev, mut pl, _maps, mut id): Self::SystemData) {
        //   println!("Received event value of: {:?}", event);
        for event in ev.read(&mut self.event_reader) {
            match event { 
                AuthEvent::Connect(auth, ip) => {
                    match authenticate(auth.to_string()) {
                        Some(s) => {
                            let player = ready_player_one(*ip, s, id.add());

                            //cmd_out.single_write(Pack::new(Cmd::InsertPlayer1(player.clone()), Dest::Ip(player.ip())));
                            //cmd_out.single_write(Pack::new(Cmd::TransferMap(player.room.clone()), Dest::Ip(player.ip())));
                            
                            io.o.push(Pack::new(Cmd::TransferMap(player.room.clone()), Dest::Ip(player.ip()))); 
                            io.o.push(Pack::new(Cmd::InsertPlayer1(player.clone()), Dest::Ip(player.ip())));
                                
                            // Push the rest of the players
                            for p in pl.list.iter() {
                                match p {
                                    Some(p) => io.o.push(Pack::new(Cmd::InsertPlayer(p.clone()), Dest::Ip(player.ip()))),
                                    //Some(p) => cmd_out.single_write(Pack::new(Cmd::InsertPlayer(p.clone()), Dest::Ip(player.ip()))),
                                    None => (),
                                }
                            }
                                
                            pl.add(player); 
                        },
                        None => (),
                    }
                }
            }
        }
    }
}

fn authenticate(proof: String) -> Option<String> {
    let v: Vec<&str> = proof.rsplit(' ').collect();

    if v.len() != 3 {
        info!("Proof Package not correct format {:?}", v); 
        return None;
    }
    
    info!("Name: {}, time: {}, Signature: {}", v[2], v[1], v[0]); 
    // Verify player here
    // |
    // |
    // Verify player here
    Some(v[2].to_string())
}

fn ready_player_one(ip: SocketAddr, name: String, id: u64) -> LifeformComponent {
    info!("Inserting player 1 ({})", name);
   
    // Dig through database to find the correct player by name = name 
    LifeformComponent::new_player(name, ip, id)
}
