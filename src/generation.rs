use bevy::{log, prelude::*};
use iyes_loopless::prelude::*;

use crate::resources::{Board, TileMap};

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::generate_map.run_unless_resource_exists::<Board>());
    }
}

impl GenerationPlugin {
    fn generate_map(mut commands: Commands) {
        log::info!("Generating map!");

        let tile_map = TileMap::new([10, 10]);

        let board_entity = commands.spawn().insert(Name::new("Board")).id();
        commands.insert_resource(Board {
            entity: board_entity,
            tile_map,
        });
    }
}
