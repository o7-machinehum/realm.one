// #![allow(
//     non_snake_case,
//     unused_must_use,
//     dead_code,
//     irrefutable_let_patterns,
//     unreachable_code
// )]
use amethyst::{
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    input::InputBundle,
    network::simulation::tcp::TcpNetworkBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use std::{fs::File, net::TcpListener};

use crate::resources::AppConfig;
use core::time::Duration;
use log::info;
use ron::de::from_reader;
use std::env;

mod components;
mod constants;
mod key_bindings;
mod map;
mod mech;
mod network;
mod resources;
mod states;
mod systems;

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
    } else {
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
        .with_bundle(TcpNetworkBundle::new(/*Some(listener)*/ None, 2048))?
        .with_bundle(systems::client::TcpSystemBundle)?
        .with(
            systems::PlayerSystem::new(config.player_name.clone()),
            "player_system",
            &["input_system"],
        )
        .with(systems::MapSystem, "map_system", &[])
        .with(systems::client::LifeformManSystem, "pm_system", &[])
        .with(systems::WalkAnimationSystem::new(), "anim_system", &[])
        .with_bundle(systems::InputSystemBundle)?
        .with(systems::MoveSystem::new(), "move_system", &[])
        .with(systems::MeleeAnimationSystem::new(), "melee_system", &[]);

    let mut game = Application::build(resources, states::GamePlayState { config })?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}

fn server(resources: std::path::PathBuf, config: AppConfig) -> amethyst::Result<()> {
    let listener = TcpListener::bind(config.server_ip.clone())?;
    listener.set_nonblocking(true)?;
    let game_data = GameDataBuilder::default()
        .with_bundle(TcpNetworkBundle::new(Some(listener), 2048))?
        .with_bundle(systems::server::TcpSystemBundle)?
        .with(systems::AuthSystem, "auth_system", &[])
        .with(systems::server::LifeformManSystem, "playerman_system", &[]);

    let mut game = Application::build(resources, states::ServerState { config })?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}
