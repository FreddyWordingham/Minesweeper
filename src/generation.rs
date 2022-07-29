use bevy::{log, prelude::*, sprite::Anchor};
use iyes_loopless::prelude::*;

use crate::{
    components::{Bomb, BombNeighbour, Coordinates},
    game::GameState,
    loading::{FontAssets, TextureAssets},
    resources::{Board, Tile, TileMap},
    settings::{self, TILE_PADDING, TILE_SIZE},
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
    fn generate_map(mut commands: Commands, fonts: Res<FontAssets>, textures: Res<TextureAssets>) {
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
                Self::spawn_tiles(parent, &tile_map, &fonts, &textures);
            })
            .id();

        commands.insert_resource(Board {
            entity: board_entity,
            tile_map,
        });
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        fonts: &Res<FontAssets>,
        textures: &Res<TextureAssets>,
    ) {
        for y in 0..tile_map.height() {
            for x in 0..tile_map.width() {
                let coords = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };

                let mut cmd = parent.spawn();
                cmd.insert(Name::new(format!("Tile ({}, {})", x, y)))
                    .insert(coords)
                    .insert_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::splat(TILE_SIZE - TILE_PADDING)),
                            ..default()
                        },
                        transform: coords.world_pos(),
                        texture: textures.tile.clone(),
                        ..default()
                    });

                match tile_map.tiles[(x, y)] {
                    Tile::Bomb => {
                        cmd.insert(Bomb);
                        cmd.with_children(|parent| {
                            parent.spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(TILE_SIZE - TILE_PADDING)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                texture: textures.bomb.clone(),
                                ..default()
                            });
                        });
                    }
                    Tile::BombNeighbor(count) => {
                        cmd.insert(BombNeighbour::new(count));
                        cmd.with_children(|parent| {
                            parent.spawn_bundle(Self::bomb_count_text_bundle(
                                count,
                                TILE_SIZE - TILE_PADDING,
                                fonts,
                            ));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    /// Generates the bomb counter text 2D Bundle for a given value.
    fn bomb_count_text_bundle(count: u8, size: f32, fonts: &Res<FontAssets>) -> Text2dBundle {
        let counter = count.saturating_sub(1) as usize;
        let colour = match vec![
            Color::PURPLE,
            Color::RED,
            Color::ORANGE,
            Color::YELLOW,
            Color::GREEN,
            Color::BLUE,
            Color::GREEN,
            Color::WHITE,
            Color::GRAY,
        ]
        .get(8 - counter)
        {
            Some(c) => *c,
            _ => Color::GRAY,
        };

        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: count.to_string(),
                    style: TextStyle {
                        color: colour,
                        font: fonts.raleway.clone(),
                        font_size: size,
                    },
                }],
                alignment: TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            },
            transform: Transform::from_xyz(0.0, TILE_PADDING, 1.0),
            ..default()
        }
    }

    fn proceed_to_menu(mut commands: Commands) {
        commands.insert_resource(NextState(GameState::Menu));
    }
}
