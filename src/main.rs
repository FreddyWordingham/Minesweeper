#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use bevy::{
    input::system::exit_on_esc_system,
    prelude::{default, App, ClearColor, Color, Msaa, WindowDescriptor},
    DefaultPlugins,
};
use minesweeper::GamePlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            title: "Minesweeper".to_string(),
            width: 800.0,
            height: 600.0,
            ..default()
        })
        .add_system(exit_on_esc_system)
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}
