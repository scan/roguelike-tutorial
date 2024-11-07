use bevy::prelude::*;

use crate::{
    board::{Position, Tile},
    loading::TextureAssets,
};

use super::{TILE_SIZE, TILE_Z};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<TextureAssets>,
) {
    for (entity, position) in query.iter() {
        commands.entity(entity).insert((
            SpriteBundle {
                texture: assets.ascii.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    color: bevy::color::palettes::css::OLIVE.into(),
                    ..default()
                },
                transform: Transform::from_translation(super::get_world_position(position, TILE_Z)),
                ..default()
            },
            TextureAtlas {
                layout: assets.ascii_layout.clone(),
                index: 177,
            },
        ));
    }
}
