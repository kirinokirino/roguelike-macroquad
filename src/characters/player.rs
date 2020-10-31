use crate::map::tiles::Tiles;
use crate::Tile;
use macroquad::Vec2;

#[derive(Debug)]
pub struct Player {
    pub tile: Tile,
}

impl Player {
    pub fn walk(&mut self, delta: Vec2) {
        self.tile.new_pos(self.tile.pos() + delta);
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            tile: Tile {
                pos: Vec2::zero(),
                kind: Tiles::Pengu,
            },
        }
    }
}
