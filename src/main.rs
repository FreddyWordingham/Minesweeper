// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{
    input::system::exit_on_esc_system,
    prelude::{default, App, ClearColor, Color, Msaa, WindowDescriptor},
    DefaultPlugins,
};
use bevy_inspector_egui::WorldInspectorPlugin;

use minesweeper::settings;
// use minesweeper::GamePlugin;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(
            Color::hex(settings::WINDOW_CLEAR_COL.split_at(1).1).unwrap(),
        ))
        .insert_resource(WindowDescriptor {
            width: settings::WINDOW_RES[0],
            height: settings::WINDOW_RES[1],
            title: settings::WINDOW_TITLE.to_string(),
            ..default()
        })
        .add_system(exit_on_esc_system)
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    // .add_plugin(GamePlugin)
    app.run();
}
