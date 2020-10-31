use macroquad::{draw_texture_ex, vec2, DrawTextureParams, Rect, Texture2D, Vec2, WHITE};
pub enum Tiles {
    Wall,
    Grass,
    Pengu,
}

impl Tiles {
    const fn value(&self) -> (f32, f32) {
        match *self {
            Self::Wall => (0., 0.),
            Self::Grass => (1., 0.),
            Self::Pengu => (2., 0.),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TileAtlas {
    texture: Texture2D,
    tile_width: f32,
    tile_height: f32,
}

impl TileAtlas {
    pub const fn new(texture: Texture2D, tile_width: f32, tile_height: f32) -> Self {
        Self {
            texture,
            tile_width,
            tile_height,
        }
    }
    pub fn draw_tile(&self, tile: &Tiles, pos: Vec2) {
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
