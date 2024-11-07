use std::ops::{Add, AddAssign, Sub, SubAssign};

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

impl Vector2Int {
    pub const UP: Self = Self { x: 0, y: 1 };
    pub const DOWN: Self = Self { x: 0, y: -1 };
    pub const LEFT: Self = Self { x: -1, y: 0 };
    pub const RIGHT: Self = Self { x: 1, y: 0 };
}

impl Add<Vector2Int> for Vector2Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<i32> for Vector2Int {
    type Output = Self;

    fn add(self, rhs: i32) -> Self {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<Vector2Int> for Vector2Int {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub<Vector2Int> for Vector2Int {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<i32> for Vector2Int {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self {
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign<Vector2Int> for Vector2Int {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
