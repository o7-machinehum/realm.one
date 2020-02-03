use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    input::InputBundle,
    utils::application_root_dir,
    network::simulation::{tcp::TcpNetworkBundle, NetworkSimulationEvent, TransportResource},
};
use std::{
   net::TcpListener,
   fs::File,
};

use ron::de::from_reader;
use crate::resources::{AppConfig};
use std::env; 
use log::info;
use core::time::Duration;

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
    let rtn: amethyst::Result<()>; 
    let app_root = application_root_dir()?;
    let resources = app_root.join("resources");

    let input_path = format!("resources/config.ron");
    let f = File::open(&input_path).expect("Failed opening file");
    let config: AppConfig = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}, using default!", e);
            AppConfig::default()
        } 
    };
    info!("{:?}", config);

    if args[1].starts_with("s") {
        info!("Starting the server!");
        rtn = server(resources, config);
    }

    else {
        info!("Starting the client");
        rtn = client(resources, config);
    }
    
    rtn
}

fn client(resources: std::path::PathBuf, config: AppConfig) -> amethyst::Result<()> {
    
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
        .with_bundle(TcpNetworkBundle::new(None, 2048))?
        .with(systems::SpamSystem::new(), "spam", &[])
        .with(systems::PlayerSystem{p1: None, timer: None, p1_name: config.player_name.clone()}, "player_system", &["input_system"])
        .with(systems::MapSystem, "map_system", &[])
        .with(systems::client::PlayerManSystem, "pm_system", &[]);
    
    let mut game = Application::build(resources, states::GamePlayState{config})?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}

fn server(resources: std::path::PathBuf, config: AppConfig) -> amethyst::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:3457")?;
    listener.set_nonblocking(true)?;
        
    let game_data = GameDataBuilder::default()
        .with_bundle(TcpNetworkBundle::new(Some(listener), 2048))?
        .with_bundle(systems::SpamReceiveBundle)?
        .with(systems::AuthSystem, "auth_system", &[])
        .with(systems::server::PlayerManSystem, "playerman_system", &[]);

    let mut game = Application::build(resources, states::ServerState{config})?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}
