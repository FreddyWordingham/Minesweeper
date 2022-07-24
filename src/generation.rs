use bevy::{log, prelude::*};
use iyes_loopless::prelude::*;

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(
        //     ConditionSet::new()
        //         .run_unless_resource_exists::<Board>()
        //         .with_system(Self::generate_board)
        //         .into(),
        // );
        app.add_system(Self::generate_board.run_unless_resource_exists::<Board>());
    }
}

impl GenerationPlugin {
    fn generate_board(mut commands: Commands) {
        log::info!("Generating board!");

        commands.insert_resource(Board {})
    }
}

struct Board {}
