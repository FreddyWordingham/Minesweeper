use bevy::{
    log,
    prelude::{App, Plugin, ResMut, State, SystemSet},
};

use crate::game::GameState;

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Generating).with_system(Self::generate_board),
        );
    }
}

impl GenerationPlugin {
    fn generate_board(mut state: ResMut<State<GameState>>) {
        log::info!("Generating board!");
        state.set(GameState::Menu).unwrap();
    }
}
