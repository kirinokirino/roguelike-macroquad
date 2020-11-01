use macroquad::{draw_texture_ex, vec2, DrawTextureParams, Rect, Texture2D, Vec2, WHITE};

/// Available kinds of Tiles. `value()` is their position on the `TileAtlas`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tiles {
    Wall,
    Grass,
    Pengu,
}

impl Tiles {
    const fn value(self) -> (f32, f32) {
        match self {
            Self::Wall => (0., 0.),
            Self::Grass => (1., 0.),
            Self::Pengu => (2., 0.),
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
    pub fn draw_tile(&self, tile: Tiles, pos: Vec2) {
        let (atlas_x, atlas_y) = tile.value();
        let params = DrawTextureParams {
            dest_size: Some(vec2(1.0, 1.0)),
            source: Some(Rect::new(
                self.tile_width * atlas_x,
                self.tile_height * atlas_y,
                self.tile_width,
                self.tile_height,
            )),
            rotation: std::f32::consts::PI,
        };
        draw_texture_ex(self.texture, pos.x(), pos.y(), WHITE, params);
    }
}
/// A position on the map with associated Tiles kind (e.g. `Tiles::Grass`)
#[derive(Debug, Clone)]
pub struct Tile {
    pub pos: Vec2,
    pub kind: Tiles,
}

impl Tile {
    pub fn new(kind: Tiles, pos: Vec2) -> Self {
        Self { pos, kind }
    }

    pub fn new_pos(&mut self, new_pos: Vec2) {
        self.pos = new_pos;
    }

    #[must_use]
    pub const fn pos(&self) -> Vec2 {
        self.pos
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            pos: Vec2::zero(),
            kind: Tiles::Wall,
        }
    }
}
