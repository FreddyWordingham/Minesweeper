#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{input::system::exit_on_esc_system, prelude::*, DefaultPlugins};
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use minesweeper::{game::GamePlugin, settings};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
    Paused,
    Playing,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(
            Color::hex(settings::WINDOW_CLEAR_COL.split_at(1).1).unwrap(),
        ))
        .insert_resource(WindowDescriptor {
            title: settings::WINDOW_TITLE.to_string(),
            width: settings::WINDOW_RES[0],
            height: settings::WINDOW_RES[1],
            ..default()
        })
        .add_system(exit_on_esc_system)
        .add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());
    app.add_startup_system(camera_setup)
        .add_startup_system(setup_world_map)
        .add_state(AppState::Paused)
        .add_plugin(GamePlugin(AppState::Playing))
        .add_system(state_handler);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_world_map(
    mut _commands: Commands,
    mut _state: ResMut<State<AppState>>,
    _asset_server: Res<AssetServer>,
) {
    println!("Set up world map");
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        if state.current() == &AppState::Paused {
            state.set(AppState::Playing).unwrap();
        } else {
            state.set(AppState::Paused).unwrap();
        }
    }
}
