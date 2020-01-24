use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::InputBundle,
    utils::application_root_dir,
    // network::simulation::{udp::UdpNetworkBundle, NetworkSimulationEvent, TransportResource},
    network::simulation::{laminar::{LaminarNetworkBundle, LaminarSocket}, NetworkSimulationEvent, TransportResource, },
};

use crate::network::{Pack};

use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::env; 
use log::info;
use core::time::Duration;
use std::net::UdpSocket;

mod map;
mod key_bindings;
mod states;
mod components;
mod systems;
mod constants;
mod mech;
mod network;
mod resources;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let args: Vec<String> = env::args().collect();
    let mut rtn : amethyst::Result<()> = Ok(()); 
    let app_root = application_root_dir()?;
    let resources = app_root.join("resources");

    if args[1] == "client" {
        info!("Starting the client");
        rtn = client(resources, args[2].clone());
    }

    else if args[1] == "server"{
        info!("Starting the server!");
        rtn = server(resources);
    }
    // else error out
    
    rtn
}

// fn get_server_config(udp_socket_addr: SocketAddr) -> ServerConfig {
//     let laminar_config = laminar::Config {
//         // blocking_mode: false,
//         idle_connection_timeout: Duration::from_millis(1000),
//         // heartbeat_interval: None,
//         max_packet_size: 16384,
//         max_fragments: 18,
//         fragment_size: 1450,
//         fragment_reassembly_buffer_size: 1450,
//         receive_buffer_max_size: 1450,
//         rtt_smoothing_factor: 0.5,
//         rtt_max_value: 500,
//         socket_event_buffer_size: 1024,
//         socket_polling_timeout: Some(Duration::from_millis(100)),
//         // max_packets_in_flight: 10,
//     };
//     
//     ServerConfig {
//         udp_socket_addr,
//         max_throughput: 5000,
//         create_net_connection_on_connect: true,
//         laminar_config,
//     }
// }

fn client(resources: std::path::PathBuf, ip: String) -> amethyst::Result<()> {
    let socket = LaminarSocket::bind("0.0.0.0:3455")?;
    
    let display_config = resources.join("display_config.ron");
    let key_bindings_config_path = resources.join("bindings.ron");
    
    let input_bundle = InputBundle::<key_bindings::MovementBindingTypes>::new()
        .with_bindings_from_file(key_bindings_config_path)?;
    
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(input_bundle)? 
        .with_bundle(LaminarNetworkBundle::new(Some(socket)))? 
        .with_bundle(systems::ClientSystemBundle)? 
        .with(systems::PlayerSystem{p1: None}, "player_system", &["input_system"])
        .with(systems::MapSystem,    "map_system", &[]);


    let mut game = Application::new(
        resources, 
        states::GamePlayState{ip},
        game_data,
    )?;

    game.run();
    Ok(())
}

fn server(resources: std::path::PathBuf) -> amethyst::Result<()> {
    let socket = LaminarSocket::bind("0.0.0.0:3456")?;
        
    let game_data = GameDataBuilder::default()
        .with_bundle(LaminarNetworkBundle::new(Some(socket)))? 
        .with_bundle(systems::ServerSystemBundle)? 
        .with(systems::AuthSystem, "auth_system", &[])
        .with(systems::PlayerManSystem{new_players: Vec::<Pack>::new()}, "playerman_system", &[]);

    let mut game = Application::new(
        resources, 
        states::ServerState{},
        game_data,
    )?;

    game.run();
    Ok(())
}
