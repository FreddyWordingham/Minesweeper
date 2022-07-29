use bevy::{log, prelude::*, sprite::Anchor};
use iyes_loopless::prelude::*;

use crate::{
    components::Coordinates,
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

        let mut tile_map = TileMap::new(settings::MAP_RES);
        tile_map.add_bombs(settings::NUM_BOMBS);

        let board_size = Vec2::new(settings::MAP_SIZE[0], settings::MAP_SIZE[1]);
        let origin = Vec2::new(0.0, 0.0);
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
            .with_children(|parent| {
                Self::spawn_tiles(parent, &tile_map);
            })
            .id();

        commands.insert_resource(Board {
            entity: board_entity,
            tile_map,
        });
    }

    fn spawn_tiles(parent: &mut ChildBuilder, tile_map: &TileMap) {
        for y in 0..tile_map.height() {
            for x in 0..tile_map.width() {
                let mut cmd = parent.spawn();
                cmd.insert(Name::new(format!("Tile ({}, {})", x, y)))
                    .insert(Coordinates {
                        x: x as u16,
                        y: y as u16,
                    });
            }
        }
    }

    fn proceed_to_menu(mut commands: Commands) {
        commands.insert_resource(NextState(GameState::Menu));
    }
}
