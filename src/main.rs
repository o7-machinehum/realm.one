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
};

mod map;
mod key_bindings;

mod states;
mod components;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");
    let key_bindings_config_path = resources.join("bindings.ron");
    
    let input_bundle = InputBundle::<key_bindings::MovementBindingTypes>::new()
        .with_bindings_from_file(key_bindings_config_path)?;

    let room = map::Room::new("resources/sprites/first.tmx".to_string());

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
        .with(systems::PlayerSystem, "player_system", &["input_system"]);


    let mut game = Application::new(
        resources, 
        states::GamePlayState{current_map: room}, 
        game_data,
    )?;

    game.run();

    Ok(())
}

