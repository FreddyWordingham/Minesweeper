use bevy::input::{mouse::MouseButtonInput, ElementState};
use bevy::log;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{Bomb, BombNeighbour, Coordinates, Uncover},
    game::GameState,
    loading::TextureAssets,
    resources::{Board, GameCamera, Tile, TileMap},
    settings::{MAP_RES, TILE_SIZE, WINDOW_SIZE},
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileTriggerEvent>()
            .add_event::<TileMarkEvent>()
            .add_event::<BombExplosionEvent>()
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(Self::input_handling)
                    .with_system(Self::trigger_event_handler)
                    .with_system(Self::uncover_tiles)
                    .into(),
            );
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TileTriggerEvent(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct TileMarkEvent(pub Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct BombExplosionEvent;

impl InputPlugin {
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn input_handling(
        windows: Res<Windows>,
        game_camera: Res<GameCamera>,
        cam: Query<&OrthographicProjection>,
        mut button_events: EventReader<MouseButtonInput>,
        mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
        mut tile_mark_ewr: EventWriter<TileMarkEvent>,
    ) {
        let window = windows.get_primary().unwrap();
        let scale = cam.get_single().unwrap().scale;

        for event in button_events.iter() {
            if ElementState::Pressed == event.state {
                let position = window.cursor_position();
                if let Some(pos) = position {
                    let window_x = pos.x - (WINDOW_SIZE[0] * 0.5);
                    let window_y = pos.y - (WINDOW_SIZE[1] * 0.5);

                    let cx = (window_x / TILE_SIZE * scale).floor() as i16;
                    let cy = (window_y / TILE_SIZE * scale).floor() as i16;

                    let x = cx + (MAP_RES[0] / 2);
                    let y = cy + (MAP_RES[1] / 2);

                    if x < 0 || y < 0 || x >= MAP_RES[0] || y >= MAP_RES[1] {
                        continue;
                    }

                    let coordinates = Coordinates { x, y } + (game_camera.1.x, game_camera.1.y);

                    match event.button {
                        MouseButton::Left => {
                            log::info!("Trying to uncover tile on {}", coordinates);
                            tile_trigger_ewr.send(TileTriggerEvent(coordinates));
                        }
                        MouseButton::Right => {
                            log::info!("Trying to mark tile on {}", coordinates);
                            tile_mark_ewr.send(TileMarkEvent(coordinates));
                        }
                        MouseButton::Middle | MouseButton::Other(_) => (),
                    }
                }
            }
        }
    }

    pub fn trigger_event_handler(
        mut commands: Commands,
        mut tile_mark_event_rdr: EventReader<TileTriggerEvent>,
        mut board: ResMut<Board>,
        textures: Res<TextureAssets>,
    ) {
        for event in tile_mark_event_rdr.iter() {
            if let Some(entity) = board.covered_tiles.get(&event.0) {
                commands.entity(*entity).insert(Uncover);
            }
        }
    }

    pub fn uncover_tiles(
        mut commands: Commands,
        mut board: ResMut<Board>,
        children: Query<(Entity, &Parent), With<Uncover>>,
        parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbour>)>,
        mut bomb_explosion_event_wr: EventWriter<BombExplosionEvent>,
    ) {
        for (entity, parent) in children.iter() {
            commands.entity(entity).despawn_recursive();

            let (coords, bomb, bomb_counter) = match parents.get(parent.0) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("{}", e);
                    continue;
                }
            };

            // match board.try_uncover_tile(coords) {
            //     None => log::debug!("Tried to uncover an already uncovered tile"),
            //     Some(e) => log::debug!("Uncovered tile {} (entity: {:?})", coords, e),
            // }

            // if board.is_completed() {
            //     log::info!("*Board completed*");
            //     board_completed_event_wr.send(BoardCompletedEvent);
            // }

            if bomb.is_some() {
                log::info!("Boom!");
                bomb_explosion_event_wr.send(BombExplosionEvent);
            }
        }
    }
}
