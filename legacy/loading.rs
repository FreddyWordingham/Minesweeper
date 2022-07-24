use bevy::prelude::{App, AssetServer, Font, Handle, HandleUntyped, Image, Mut, Plugin, World};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::game::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_loopless_state(GameState::Loading)
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Generating)
                    .with_collection::<FontAssets>()
                    .with_collection::<AudioAssets>()
                    .with_collection::<TextureAssets>(),
            );
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/raleway.ttf")]
    pub raleway: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/explosion.ogg")]
    pub explosion: Handle<AudioSource>,

    #[asset(path = "audio/select_1.ogg")]
    pub select_1: Handle<AudioSource>,

    #[asset(path = "audio/select_2.ogg")]
    pub select_2: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "icons/bomb.png")]
    pub bomb: Handle<Image>,

    #[asset(path = "icons/flag.png")]
    pub flag: Handle<Image>,
}
