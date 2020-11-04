pub mod generators;
pub mod tiles;

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
