use crate::map::tiles::{Position, Tile};
use macroquad::vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsPlayer {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mover {
    pub x: i32,
    pub y: i32,
}

impl Mover {
    pub fn try_move(&mut self, delta: (i32, i32)) {
        let (x, y) = delta;
        self.x = x;
        self.y = y;
    }
}
