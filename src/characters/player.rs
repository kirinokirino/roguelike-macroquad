use crate::map::tiles::Position;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IsPlayer {}

#[derive(Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<Position>,
    pub range: i32,
}
