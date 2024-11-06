mod board;
mod camera;
mod globals;
mod graphics;
mod loading;
mod pieces;
mod states;
mod vectors;

use bevy::prelude::*;

use board::BoardPlugin;
use graphics::GraphicsPlugin;
use loading::LoadingPlugin;
use states::GameState;

pub use globals::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((LoadingPlugin, BoardPlugin, GraphicsPlugin))
            .add_systems(OnEnter(GameState::Playing), camera::setup);
    }
}
