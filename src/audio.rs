use crate::loading::AudioAssets;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioPlugin as KiraAudioPlugin};

use crate::game::GameState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(KiraAudioPlugin)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(Self::start_audio))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(Self::stop_audio));
    }
}

impl AudioPlugin {
    fn start_audio(_audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
        audio.set_volume(0.1);
        // audio.play_looped(audio_assets.explosion.clone());
        audio.resume();
    }

    fn stop_audio(audio: Res<Audio>) {
        audio.stop();
    }
}
