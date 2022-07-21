#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, WindowDescriptor, *};
use bevy::DefaultPlugins;
use minesweeper::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "Minesweeper".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}
