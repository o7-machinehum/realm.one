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
    network::NetworkBundle,
};

use std::env; 
use log::info;

mod map;
mod key_bindings;
mod states;
mod components;
mod systems;
mod constants;
mod mech;
mod network;
mod resources;
mod events;

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

fn client(resources: std::path::PathBuf, ip: String) -> amethyst::Result<()> {
    let display_config = resources.join("display_config.ron");
    let key_bindings_config_path = resources.join("bindings.ron");
    
    let input_bundle = InputBundle::<key_bindings::MovementBindingTypes>::new()
        .with_bindings_from_file(key_bindings_config_path)?;
    
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(input_bundle)? 
        .with_bundle(NetworkBundle::<Vec<u8>>::new(
            "127.0.0.1:3455".parse().unwrap(),
        ))?
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(systems::ClientSystem, "client_system", &[]);


    let mut game = Application::new(
        resources, 
        states::GamePlayState{ip},
        game_data,
    )?;

    game.run();
    Ok(())
}

fn server(resources: std::path::PathBuf) -> amethyst::Result<()> {
    let game_data = GameDataBuilder::default()
        .with_bundle(NetworkBundle::<Vec<u8>>::new(
            "127.0.0.1:3456".parse().unwrap(),
        ))?
        .with(systems::ServerSystem, "server_system", &[]);

    let mut game = Application::new(
        resources, 
        states::ServerState{},
        game_data,
    )?;

    game.run();
    Ok(())
}
