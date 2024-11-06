use bevy::{prelude::*, utils::HashMap};

use super::{
    components::{Position, Tile},
    CurrentBoard,
};

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentBoard>) {
    current.tiles = HashMap::new();
    for x in 0..8 {
        for y in 0..8 {
            let v = (x, y).into();
            let tile = commands
                .spawn((
                    Position { v },
                    Tile,
                    Name::new(format!("Tile {}, {}", x, y)),
                ))
                .id();
            current.tiles.insert(v, tile);
        }
    }
}
