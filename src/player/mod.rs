use bevy::prelude::*;

use crate::{board::Position, pieces::components::Piece, states::GameState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Piece {
            kind: "Player".to_string(),
        },
        Position { v: (0, 0).into() },
    ));
}
