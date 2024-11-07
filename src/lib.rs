mod actions;
mod board;
mod camera;
mod globals;
mod graphics;
mod input;
mod loading;
mod pieces;
mod player;
mod states;
mod vectors;

use bevy::prelude::*;

use actions::ActionsPlugin;
use board::BoardPlugin;
use graphics::GraphicsPlugin;
use input::InputPlugin;
use loading::LoadingPlugin;
use player::PlayerPlugin;
use states::GameState;

pub use globals::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_plugins((
                ActionsPlugin,
                BoardPlugin,
                GraphicsPlugin,
                InputPlugin,
                LoadingPlugin,
                PlayerPlugin,
            ))
            .add_systems(OnEnter(GameState::Playing), camera::setup);
    }
}
