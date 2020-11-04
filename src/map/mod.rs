pub mod generators;
pub mod tiles;
use crate::map::tiles::Position;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 1, y: 1 }
    }
}

impl From<Position> for Point {
    fn from(pos: Position) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

pub struct Rect {
    pub top_left: Point,
    pub down_right: Point,
}

impl Rect {
    pub const fn new(top_left: Point, width: i32, height: i32) -> Self {
        Self {
            top_left,
            down_right: Point {
                x: top_left.x + width,
                y: top_left.y + height,
            },
        }
    }

    pub const fn _from_points(top_left: Point, down_right: Point) -> Self {
        Self {
            top_left,
            down_right,
        }
    }

    /// Returns the outer layer of the rectangle.
    pub fn get_walls_positions(&self) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for x in self.top_left.x - 1..=self.down_right.x {
            for y in self.top_left.y - 1..=self.down_right.y {
                positions.push((x as usize, y as usize));
            }
        }
        positions
    }

    /// Returns the inner rectangle behind the wall.
    pub fn get_floors_positions(&self) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::new();
        for x in self.top_left.x..self.down_right.x {
            for y in self.top_left.y..self.down_right.y {
                positions.push((x as usize, y as usize));
            }
        }
        positions
    }

    // Returns true if this overlaps with other
    pub const fn intersect(&self, other: &Self) -> bool {
        self.top_left.x <= other.down_right.x
            && self.down_right.x >= other.top_left.x
            && self.top_left.y <= other.down_right.y
            && self.down_right.y >= other.top_left.y
    }

    pub const fn center(&self) -> Point {
        Point {
            x: (self.top_left.x + self.down_right.x) / 2,
            y: (self.top_left.y + self.down_right.y) / 2,
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            top_left: Point::default(),
            down_right: Point::default(),
        }
    }
}
