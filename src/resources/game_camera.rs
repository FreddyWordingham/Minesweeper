//! Game world camera.

use bevy::prelude::*;

use crate::components::Coordinates;

/// Game world camera.
#[derive(Debug)]
pub struct GameCamera(pub Entity, pub Coordinates);
