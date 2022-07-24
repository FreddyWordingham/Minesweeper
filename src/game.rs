use bevy::{log, prelude::*};
use iyes_loopless::prelude::*;

use crate::{generation::GenerationPlugin, loading::LoadingPlugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Generating,
    Menu,
    Playing,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::Loading)
            .add_plugin(GenerationPlugin)
            .add_plugin(LoadingPlugin)
            .add_system(Self::test_system)
            .add_system(Self::test_system_playing.run_in_state(GameState::Playing));
    }
}

impl GamePlugin {
    #[allow(clippy::needless_pass_by_value)]
    fn test_system(mut _commands: Commands, keys: Res<Input<KeyCode>>) {
        if keys.just_pressed(KeyCode::Space) {
            log::info!("[SPACED!]");
        }
    }

    fn test_system_playing() {
        log::info!("playing");
    }
}
