mod tiles;

use bevy::prelude::*;

use crate::states::GameState;

pub const TILE_SIZE: f32 = 32.;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            tiles::spawn_tile_renderer.run_if(in_state(GameState::Playing)),
        );
    }
}
