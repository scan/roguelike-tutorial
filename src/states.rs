use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum MainState {
    #[default]
    Loading,
    Playing,
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    TurnUpdate
}
