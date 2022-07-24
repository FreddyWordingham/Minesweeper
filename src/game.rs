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
                FixedTimestepStage::from_stage(Duration::from_millis(125), fixed_update),
            )
            .add_plugin(GenerationPlugin)
            .add_plugin(LoadingPlugin)
            .add_system(Self::test_system)
            .add_enter_system(GameState::Playing, Self::add_camera);
    }
}

impl GamePlugin {
    #[allow(clippy::needless_pass_by_value)]
    fn test_system(
        mut commands: Commands,
        current_state: Res<CurrentState<GameState>>,
        keys: Res<Input<KeyCode>>,
    ) {
        if keys.just_pressed(KeyCode::Return) {
            log::info!("[ENTER!] {:?}", *current_state);
        } else if keys.just_pressed(KeyCode::Space) {
            log::info!("[SPACED!]");
            match current_state.0 {
                GameState::Playing => {
                    log::info!("Switching to menu state");
                    commands.insert_resource(NextState(GameState::Menu));
                }
                GameState::Menu => {
                    log::info!("Switching to playing state");
                    commands.insert_resource(NextState(GameState::Playing));
                }
                _ => {}
            }
        }
    }

    fn test_system_playing() {
        log::info!("playing");
    }

    fn add_camera(mut commands: Commands) {
        commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    }
}
