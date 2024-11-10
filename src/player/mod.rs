use bevy::prelude::*;

use crate::{
    board::Position,
    pieces::components::{Actor, Piece},
    states::MainState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Playing), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Actor::default(),
        Player,
        Piece {
            kind: "Player".to_string(),
        },
        Position { v: (0, 0).into() },
    ));
}
