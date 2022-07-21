#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    input::system::exit_on_esc_system,
    prelude::{default, App, ClearColor, Color, Msaa, WindowDescriptor},
    DefaultPlugins,
};
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use std::path::Path;

use minesweeper::{game::GamePlugin, parameters::Parameters, parse::json};
fn main() {
    // args!(_bin_path: PathBuf, params_path: PathBuf);
    // let params: Parameters = json::load(&params_path);
    let params: Parameters = json::load(Path::new("parameters.json"));
    println!("Parameters {:?}!", params);

    let mut app = App::new();

    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(
            Color::hex(params.window_clear_col.split_at(1).1).unwrap(),
        ))
        .insert_resource(WindowDescriptor {
            title: params.window_title,
            width: params.window_res[0],
            height: params.window_res[1],
            ..default()
        })
        .add_system(exit_on_esc_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
