use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::Coordinates,
    game::GameState,
    resources::{GameCamera, UiCamera},
    settings,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(Self::zoom_camera)
                //.with_system(Self::pan_camera)
                .with_system(Self::pan_world)
                .into(),
        )
        .add_enter_system(GameState::Playing, Self::add_camera)
        .add_exit_system(GameState::Playing, Self::remove_camera)
        .add_enter_system(GameState::Menu, Self::add_ui_camera)
        .add_exit_system(GameState::Menu, Self::remove_ui_camera);
    }
}

impl CameraPlugin {
    fn add_camera(mut commands: Commands) {
        let mut cam = OrthographicCameraBundle::new_2d();
        cam.transform.translation.x = settings::MAP_SIZE[0] * 0.5;
        cam.transform.translation.y = settings::MAP_SIZE[1] * 0.5;

        let camera_entity = commands.spawn_bundle(cam).id();
        commands.insert_resource(GameCamera(camera_entity, Coordinates { x: 0, y: 0 }));
    }

    #[allow(clippy::needless_pass_by_value)]
    fn remove_camera(mut commands: Commands, game_camera: Res<GameCamera>) {
        commands.entity(game_camera.0).despawn_recursive();
    }

    fn add_ui_camera(mut commands: Commands) {
        let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();
        commands.insert_resource(UiCamera(camera_entity));
    }

    #[allow(clippy::needless_pass_by_value)]
    fn remove_ui_camera(mut commands: Commands, game_camera: Res<UiCamera>) {
        commands.entity(game_camera.0).despawn_recursive();
    }

    #[allow(clippy::needless_pass_by_value)]
    fn zoom_camera(keys: Res<Input<KeyCode>>, mut cam: Query<&mut OrthographicProjection>) {
        let mut cam = cam.single_mut();

        if keys.just_pressed(KeyCode::Equals) {
            cam.scale *= settings::CAMERA_ZOOM_SPEED;
        } else if keys.just_pressed(KeyCode::Minus) {
            cam.scale /= settings::CAMERA_ZOOM_SPEED;
        }

        if cam.scale > settings::CAMERA_MAX_ZOOM {
            cam.scale = settings::CAMERA_MAX_ZOOM;
        } else if cam.scale < settings::CAMERA_MIN_ZOOM {
            cam.scale = settings::CAMERA_MIN_ZOOM;
        }
    }

    #[allow(clippy::needless_pass_by_value)]
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
    }

    #[allow(clippy::needless_pass_by_value)]
    fn pan_world(
        keys: Res<Input<KeyCode>>,
        mut camera: ResMut<GameCamera>,
        mut coords: Query<(&mut Transform, &mut Coordinates)>,
    ) {
        let mut dx = 0;
        let mut dy = 0;
        if keys.pressed(KeyCode::W) {
            dy -= 1;
        }
        if keys.pressed(KeyCode::A) {
            dx += 1;
        }
        if keys.pressed(KeyCode::S) {
            dy += 1;
        }
        if keys.pressed(KeyCode::D) {
            dx -= 1;
        }

        if dx == 0 && dy == 0 {
            return;
        }

        camera.1 = camera.1 + (-dx, -dy);
        for (mut pos, mut coords) in coords.iter_mut() {
            *coords = *coords + (dx, dy);
            pos.translation = coords.world_pos();
        }
    }
}
