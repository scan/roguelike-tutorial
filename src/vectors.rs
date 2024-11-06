use bevy::reflect::Reflect;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct Vector2Int {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Vector2Int {
    fn from(v: (i32, i32)) -> Self {
        Self { x: v.0, y: v.1 }
    }
}
