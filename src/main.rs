#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::clone_on_ref_ptr,
    clippy::else_if_without_else,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::mem_forget,
    clippy::multiple_inherent_impl,
    clippy::option_unwrap_used,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::result_unwrap_used,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::string_add,
    clippy::wildcard_enum_match_arm,
    clippy::wrong_pub_self_convention
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::unknown_clippy_lints,
    clippy::option_expect_used,
    clippy::result_expect_used
)]

use legion::*;
use macroquad::*;

const TILE_WIDTH: f32 = 16.;
const TILE_HEIGHT: f32 = 16.;
#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    pos: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Acceleration {
    acc: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Camera {
    target: Vec2,
    zoom: Vec2,
}

fn draw_ui() {
    // Screen space, render fixed ui
    set_default_camera();
    let text_color: Color = Color([100, 100, 100, 150]);
    draw_text(",aoe to move camera", 10.0, 0.0, 30.0, text_color);
    draw_text(
        "PageUp and PageDown to zoom camera",
        10.0,
        50.0,
        30.0,
        text_color,
    );
}

fn move_camera(camera: &mut Camera) {
    // scroll
    if is_key_down(KeyCode::Comma) {
        camera
            .target
            .set_y(camera.target.y() + 0.01 / camera.zoom.x())
    }
    if is_key_down(KeyCode::O) {
        camera
            .target
            .set_y(camera.target.y() - 0.01 / camera.zoom.x())
    }
    if is_key_down(KeyCode::A) {
        camera
            .target
            .set_x(camera.target.x() - 0.01 / camera.zoom.x())
    }
    if is_key_down(KeyCode::E) {
        camera
            .target
            .set_x(camera.target.x() + 0.01 / camera.zoom.x())
    }
    // zoom
    if is_key_down(KeyCode::PageUp) {
        camera.zoom.set_x(camera.zoom.x() * 0.98);
        camera.zoom.set_y(camera.zoom.y() * 0.98);
    }
    if is_key_down(KeyCode::PageDown) {
        camera.zoom.set_x(camera.zoom.x() / 0.98);
        camera.zoom.set_y(camera.zoom.y() / 0.98);
    }
}

fn get_relative_mouse_position(camera: &Camera) -> Vec2 {
    let mouse = mouse_position();
    Vec2::new(
        ((mouse.0 - screen_width() / 2.0) / (screen_width() / 2.0) / camera.zoom.x())
            + camera.target.x(),
        ((-mouse.1 + screen_height() / 2.0)
            / (screen_height() / 2.0)
            / camera.zoom.x()
            / (screen_width() / screen_height()))
            + camera.target.y(),
    )
}

enum Tile {
    BlueFloor,
}

impl Tile {
    const fn value(&self) -> (f32, f32) {
        match *self {
            Tile::BlueFloor => (1., 4.),
        }
    }
}

struct TileAtlas {
    texture: Texture2D,
    tile_width: f32,
    tile_height: f32,
}

impl TileAtlas {
    fn draw_tile(&self, tile: Tile, x: f32, y: f32) {
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
        draw_texture_ex(self.texture, x, y, WHITE, params);
    }
}

#[macroquad::main("Name")]
async fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();
    let texture = load_texture("Floor.png").await;
    let atlas = TileAtlas {
        texture,
        tile_width: TILE_WIDTH,
        tile_height: TILE_HEIGHT,
    };

    let starting_zoom = 0.05;
    let mut main_camera = Camera {
        target: vec2(0.0, 0.0),
        zoom: vec2(
            starting_zoom,
            starting_zoom * screen_width() / screen_height(),
        ),
    };

    let mut mouse_pressed = false;

    // construct a schedule (you should do this on init)
    let mut schedule = Schedule::builder().build();

    loop {
        // Update

        let mouse_position = get_relative_mouse_position(&main_camera);
        move_camera(&mut main_camera);
        if is_key_down(KeyCode::Right) {}
        if is_key_down(KeyCode::Left) {}
        if is_key_down(KeyCode::Down) {}
        if is_key_down(KeyCode::Up) {}
        if is_mouse_button_down(MouseButton::Left) {
            if mouse_pressed == false {
                let pos = get_relative_mouse_position(&main_camera);
                info!("Mouse pressed at x:{} , y:{}", pos.x(), pos.y())
            }
            mouse_pressed = true;
        } else {
            mouse_pressed = false;
        }

        // Draw

        clear_background(Color([255, 255, 255, 255]));

        // Camera space, render game objects
        set_camera(Camera2D {
            target: main_camera.target,
            zoom: main_camera.zoom,
            ..Default::default()
        });

        atlas.draw_tile(Tile::BlueFloor, 0., 0.);
        draw_circle(
            mouse_position.x(),
            mouse_position.y(),
            0.2,
            Color([200, 150, 225, 255]),
        );

        draw_ui();

        // run our schedule (you should do this each update)
        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
