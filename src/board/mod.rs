use bevy::{prelude::*, utils::HashMap};

use crate::{states::GameState, vectors::Vector2Int};

mod components;
mod systems;

pub use components::*;

#[derive(Default, Debug, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .register_type::<components::Position>()
            .register_type::<components::Tile>()
            .add_systems(OnEnter(GameState::Playing), systems::spawn_map);
    }
}
