#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    app::App,
    ecs::schedule::StateData,
    prelude::{Commands, Plugin, Res, SystemSet, WindowDescriptor},
};

pub struct GamePlugin<T>(pub T);

impl<T: StateData> Plugin for GamePlugin<T> {
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }

        app.add_system_set(SystemSet::on_enter(self.0.clone()).with_system(Self::enter_running));
        app.add_system_set(SystemSet::on_update(self.0.clone()).with_system(Self::update_running));
        app.add_system_set(SystemSet::on_exit(self.0.clone()).with_system(Self::exit_running));
    }
}

impl<T> GamePlugin<T> {
    fn enter_running(mut _commands: Commands, _window: Res<WindowDescriptor>) {
        println!("Hello, world!");
    }

    fn update_running(mut _commands: Commands, _window: Res<WindowDescriptor>) {
        println!("Running...");
    }

    fn exit_running(mut _commands: Commands, _window: Res<WindowDescriptor>) {
        println!("Goodbye, world!");
    }
}
