use bevy::{log, prelude::*};
use iyes_loopless::prelude::*;

use crate::{
    game::GameState,
    resources::{Board, TileMap},
};

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Generating)
                .with_system(Self::generate_map.run_unless_resource_exists::<Board>())
                .with_system(Self::proceed_to_menu.run_if_resource_exists::<Board>())
                .into(),
        );
    }
}

impl GenerationPlugin {
    fn generate_map(mut commands: Commands) {
        log::info!("Generating map!");

        let mut tile_map = TileMap::new([40, 20]);
        tile_map.add_bombs(40);

        let board_size = Vec2::new(40.0, 30.0);

        let board_entity = commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::default())
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::PURPLE,
                            custom_size: Some(board_size),
                            ..default()
                        },
                        transform: Transform::from_xyz(board_size.x * 0.5, board_size.y * 0.5, 0.0),
                        ..default()
                    })
                    .insert(Name::new("Background"));
            })
            .id();

        commands.insert_resource(Board {
            entity: board_entity,
            tile_map,
        });
    }

    fn proceed_to_menu(mut commands: Commands) {
        commands.insert_resource(NextState(GameState::Menu));
    }
}
