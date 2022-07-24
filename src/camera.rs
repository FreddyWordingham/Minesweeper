use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{game::GameState, settings};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Self::zoom_camera)
                .with_system(Self::pan_camera)
                .into(),
        );
    }
}

impl CameraPlugin {
    fn zoom_camera(keys: Res<Input<KeyCode>>, mut cam: Query<&mut OrthographicProjection>) {
        let mut cam = cam.single_mut();

        if keys.just_pressed(KeyCode::Equals) {
            cam.scale *= settings::CAMERA_ZOOM_SPEED;
        } else if keys.just_pressed(KeyCode::Minus) {
            cam.scale /= settings::CAMERA_ZOOM_SPEED;
        }
    }

    fn pan_camera(
        keys: Res<Input<KeyCode>>,
        mut cam: Query<(&mut Transform, &mut OrthographicProjection)>,
    ) {
        let (mut pos, cam) = cam.single_mut();

        let mut move_delta = Vec2::new(0.0, 0.0);
        if keys.pressed(KeyCode::W) {
            move_delta.y += settings::CAMERA_PAN_SPEED;
        }
        if keys.pressed(KeyCode::A) {
            move_delta.x -= settings::CAMERA_PAN_SPEED;
        }
        if keys.pressed(KeyCode::S) {
            move_delta.y -= settings::CAMERA_PAN_SPEED;
        }
        if keys.pressed(KeyCode::D) {
            move_delta.x += settings::CAMERA_PAN_SPEED;
        }

        pos.translation += Vec3::new(move_delta.x * cam.scale, move_delta.y * cam.scale, 0.0);

        bevy::log::info!("Move delta: {:?}", move_delta);
    }
}
