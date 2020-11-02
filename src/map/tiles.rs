use macroquad::{draw_texture_ex, vec2, DrawTextureParams, Rect, Texture2D, Vec2, WHITE};
/// Available kinds of Tiles. `value()` is their position on the `TileAtlas`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Wall,
    Grass,
    Pengu,
}

impl Tile {
    const fn value(self) -> (f32, f32) {
        match self {
            Self::Wall => (0., 0.),
            Self::Grass => (1., 0.),
            Self::Pengu => (2., 0.),
        }
    }
    const fn is_walkable(self) -> bool {
        match self {
            Self::Wall => false,
            Self::Grass => true,
            Self::Pengu => false,
        }
    }
}

/// Is used to split one `Texture2D` into different tiles.
#[derive(Clone, Debug)]
pub struct TileAtlas {
    texture: Texture2D,
    tile_width: f32,
    tile_height: f32,
}

impl TileAtlas {
    /// Initialize the atlas from the texture and tile size.
    pub const fn new(texture: Texture2D, tile_width: f32, tile_height: f32) -> Self {
        Self {
            texture,
            tile_width,
            tile_height,
        }
    }

    /// Draw provided Tiles kind (e.g. `Tiles::Grass`) to the given position.
    pub fn draw_tile(&self, tile: &Tile, pos: &Position) {
        let (atlas_position_x, atlas_position_y) = tile.value();
        let params = DrawTextureParams {
            dest_size: Some(Vec2::one()),
            source: Some(Rect {
                x: self.tile_width * atlas_position_x,
                y: self.tile_height * atlas_position_y,
                w: self.tile_width,
                h: self.tile_height,
            }),
            rotation: std::f32::consts::PI,
        };
        draw_texture_ex(self.texture, pos.x as f32, pos.y as f32, WHITE, params);
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct AtlasPosition {
    x: f32,
    y: f32,
}
