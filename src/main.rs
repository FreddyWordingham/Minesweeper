// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #[cfg(feature = "debug")]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{input::system::exit_on_esc_system, prelude::*, DefaultPlugins};
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;

use minesweeper::{settings, GamePlugin};

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa {
        samples: settings::WINDOW_SAMPLES,
    })
    .insert_resource(ClearColor(
        Color::hex(settings::WINDOW_CLEAR_COL.split_at(1).1).unwrap(),
    ))
    .insert_resource(WindowDescriptor {
        width: settings::WINDOW_SIZE[0],
        height: settings::WINDOW_SIZE[1],
        title: settings::WINDOW_TITLE.to_string(),
        ..default()
    })
    .add_system(exit_on_esc_system)
    .add_plugins(DefaultPlugins)
    .add_plugin(AudioPlugin);

    #[cfg(feature = "debug")]
    {
        app.add_plugin(WorldInspectorPlugin::new());
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(LogDiagnosticsPlugin::default());
    }

    app.add_plugin(GamePlugin).run();
}
