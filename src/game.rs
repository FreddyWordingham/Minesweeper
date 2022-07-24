use bevy::{log, prelude::*, utils::Duration};
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
    #[inline]
    fn build(&self, app: &mut App) {
        let mut fixed_update = SystemStage::parallel();
        fixed_update.add_system(Self::test_system_playing.run_in_state(GameState::Playing));

        app.add_loopless_state(GameState::Loading)
            .add_stage_before(
                CoreStage::Update,
                "FixedUpdate",
                FixedTimestepStage::from_stage(Duration::from_millis(1025), fixed_update),
            )
            .add_plugin(GenerationPlugin)
            .add_plugin(LoadingPlugin)
            .add_system(Self::test_system);
    }
}

impl GamePlugin {
    #[allow(clippy::needless_pass_by_value)]
    fn test_system(mut commands: Commands, keys: Res<Input<KeyCode>>) {
        if keys.just_pressed(KeyCode::Space) {
            log::info!("[SPACED!]");
            commands.insert_resource(NextState(GameState::Playing));
        }
    }

    fn test_system_playing() {
        log::info!("playing");
    }
}
