use bevy::prelude::*;

use crate::{
    board::{Position, Tile},
    loading::TextureAssets,
};

use super::TILE_SIZE;

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
                transform: Transform::from_translation(Vec3::new(
                    position.v.x as f32 * TILE_SIZE,
                    position.v.y as f32 * TILE_SIZE,
                    0.0,
                )),
                ..default()
            },
            TextureAtlas {
                layout: assets.ascii_layout.clone(),
                index: 177,
            },
        ));
    }
}
