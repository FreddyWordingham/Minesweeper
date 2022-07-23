use bevy::prelude::{App, AssetServer, Font, Handle, HandleUntyped, Image, Plugin, World};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

use crate::game::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .continue_to_state(GameState::Menu)
            .build(app);
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
