use bevy::{log, prelude::*, sprite::Anchor};
use iyes_loopless::prelude::*;

use crate::{
    game::GameState,
    resources::{Board, TileMap},
    settings,
    utility::Bounds,
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

        let board_size = Vec2::new(settings::WINDOW_RES[0], settings::WINDOW_RES[1]);
        let origin = Vec2::new(
            -0.5 * settings::WINDOW_RES[0],
            -0.5 * settings::WINDOW_RES[1],
        );
        let board_bounds = Bounds {
            mins: origin,
            maxs: origin + board_size,
        };

        let board_entity = commands
            .spawn()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(Vec3::new(
                board_bounds.mins.x,
                board_bounds.mins.y,
                0.0,
            )))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::hex(settings::BOARD_COL.split_at(1).1).unwrap(),
                            custom_size: Some(board_bounds.maxs - board_bounds.mins),
                            anchor: Anchor::BottomLeft,
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
