use bevy::{log, prelude::*, utils::Duration};
#[cfg(feature = "debug")]
use bevy_inspector_egui::RegisterInspectable;
use iyes_loopless::prelude::*;

#[cfg(feature = "debug")]
use crate::components::Coordinates;
use crate::{
    camera::CameraPlugin, generation::GenerationPlugin, input::InputPlugin, loading::LoadingPlugin,
    menu::MenuPlugin,
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
        #[cfg(feature = "debug")]
        {
            app.register_inspectable::<Coordinates>();
        }

        let mut fixed_update = SystemStage::parallel();
        // fixed_update.add_system(InputPlugin::uncover_tiles.run_in_state(GameState::Playing));

        app.add_loopless_state(GameState::Loading)
            .add_stage_before(
                CoreStage::Update,
                "FixedUpdate",
                FixedTimestepStage::from_stage(Duration::from_millis(1000), fixed_update),
            )
            .add_plugin(LoadingPlugin)
            .add_plugin(GenerationPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InputPlugin);
    }
}
