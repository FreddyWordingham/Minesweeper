use bevy::{log, prelude::*, utils::Duration};
use iyes_loopless::prelude::*;

use crate::{
    camera::CameraPlugin, generation::GenerationPlugin, loading::LoadingPlugin,
    resources::GameCamera, settings,
};

const MIN_ZOOM: f32 = 0.1;
const MAX_ZOOM: f32 = 1000.0;
const ZOOM_SPEED: f32 = 0.1;

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
        fixed_update
            .add_system(Self::test_system_loading.run_in_state(GameState::Loading))
            .add_system(Self::test_system_generating.run_in_state(GameState::Generating))
            .add_system(Self::test_system_menu.run_in_state(GameState::Menu))
            .add_system(Self::test_system_playing.run_in_state(GameState::Playing));

        app.add_loopless_state(GameState::Loading)
            .add_stage_before(
                CoreStage::Update,
                "FixedUpdate",
                FixedTimestepStage::from_stage(Duration::from_millis(1000), fixed_update),
            )
            .add_plugin(LoadingPlugin)
            .add_plugin(GenerationPlugin)
            .add_plugin(CameraPlugin)
            .add_system(Self::test_system)
            .add_enter_system(GameState::Playing, Self::add_camera)
            .add_exit_system(GameState::Playing, Self::remove_camera)
            .add_enter_system(GameState::Menu, Self::add_camera)
            .add_exit_system(GameState::Menu, Self::remove_camera);
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
                _ => {
                    log::info!("Can not switch states from {:?}", current_state.0);
                }
            }
        }
    }

    fn add_camera(mut commands: Commands) {
        let mut cam = OrthographicCameraBundle::new_2d();
        cam.transform.translation.x = settings::MAP_SIZE[0] * 0.5;
        cam.transform.translation.y = settings::MAP_SIZE[1] * 0.5;

        let camera_entity = commands.spawn_bundle(cam).id();
        commands.insert_resource(GameCamera(camera_entity))
    }

    fn remove_camera(mut commands: Commands, game_camera: Res<GameCamera>) {
        commands.entity(game_camera.0).despawn_recursive();
    }

    fn test_system_loading() {
        log::info!("** loading **");
    }

    fn test_system_generating() {
        log::info!("** generating **");
    }

    fn test_system_menu() {
        log::info!("** menu **");
    }

    fn test_system_playing() {
        log::info!("** playing **");
    }
}
