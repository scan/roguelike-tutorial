use bevy::prelude::*;

use crate::graphics::TILE_SIZE;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            4. * TILE_SIZE,
            4. * TILE_SIZE,
            0.0,
        )),
        ..default()
    });
}
