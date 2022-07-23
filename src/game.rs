use bevy::{
    log,
    prelude::{
        App, Commands, Input, KeyCode, Plugin, Res, ResMut, State, SystemSet, WindowDescriptor,
    },
};

use crate::{audio::AudioPlugin, loading::LoadingPlugin, menu::MenuPlugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(MenuPlugin)
            .add_system(Self::state_handler)
            .add_system_set(
                SystemSet::on_enter(GameState::Playing).with_system(Self::enter_running),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(Self::update_running),
            )
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(Self::exit_running));
    }
}

impl GamePlugin {
    #[allow(clippy::needless_pass_by_value)]
    fn enter_running(mut _commands: Commands, _window: Res<WindowDescriptor>) {
        log::info!("Hello, world!");
    }

    #[allow(clippy::needless_pass_by_value)]
    fn update_running(mut _commands: Commands, _window: Res<WindowDescriptor>) {
        log::info!("Running...");
    }

    #[allow(clippy::needless_pass_by_value)]
    fn exit_running(mut _commands: Commands, _window: Res<WindowDescriptor>) {
        log::info!("Goodbye, world!");
    }

    #[allow(clippy::needless_pass_by_value)]
    fn state_handler(mut state: ResMut<State<GameState>>, keys: Res<Input<KeyCode>>) {
        if keys.just_pressed(KeyCode::Space) {
            if state.current() == &GameState::Menu {
                state.set(GameState::Playing).unwrap();
            } else {
                state.set(GameState::Menu).unwrap();
            }
        }
    }
}
