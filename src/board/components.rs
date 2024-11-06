use bevy::prelude::*;
use bevy::reflect::Reflect;

use crate::vectors::Vector2Int;

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct Position {
    pub v: Vector2Int,
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
pub struct Tile;
