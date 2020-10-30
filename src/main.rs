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

mod map;
use crate::map::tiles::{TileAtlas, Tiles};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    pos: Vec2,
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct Camera {
    target: Vec2,
    zoom: Vec2,
}

/// Render the fixed screen ui. (after set_default_camera())
fn draw_ui() {
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

/// Get and handle the input related to the camera.
fn move_camera(camera: &mut Camera) {
    // Move the camera:
    // UP
    if is_key_down(KeyCode::Comma) {
        camera
            .target
            .set_y(camera.target.y() + 0.01 / camera.zoom.x())
    }
    // DOWN
    if is_key_down(KeyCode::O) {
        camera
            .target
            .set_y(camera.target.y() - 0.01 / camera.zoom.x())
    }
    // LEFT
    if is_key_down(KeyCode::A) {
        camera
            .target
            .set_x(camera.target.x() - 0.01 / camera.zoom.x())
    }
    // RIGHT
    if is_key_down(KeyCode::E) {
        camera
            .target
            .set_x(camera.target.x() + 0.01 / camera.zoom.x())
    }
    // Change the camera zoom:
    // Further
    if is_key_down(KeyCode::PageUp) {
        camera.zoom.set_x(camera.zoom.x() * 0.98);
        camera.zoom.set_y(camera.zoom.y() * 0.98);
    }
    // Closer
    if is_key_down(KeyCode::PageDown) {
        camera.zoom.set_x(camera.zoom.x() / 0.98);
        camera.zoom.set_y(camera.zoom.y() / 0.98);
    }
}

/// Get the mouse coordinates inside the game world.
fn relative_mouse_position(camera: &Camera) -> Vec2 {
    // Takes the mouse coordinates on window and translates that
    // to game world coordinates.
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
#[macroquad::main("Name")]
async fn main() {
    // Init world and resources of legion ECS.
    let mut world = World::default();
    let mut resources = Resources::default();

    // Construct a systems schedule of legion ECS.
    let mut schedule = Schedule::builder().build();

    // Load assets.
    let texture = load_texture("assets/Tiles.png").await;

    // Construct TileAtlas.
    let atlas = TileAtlas::new(texture, 32., 32.);
    resources.insert(atlas.clone());

    // Main camera.
    let starting_zoom = 0.05;
    let mut main_camera = Camera {
        target: vec2(0.0, 0.0),
        zoom: vec2(
            starting_zoom,
            starting_zoom * screen_width() / screen_height(),
        ),
    };

    // We need to save the state of the mouse button
    // to detect mouse clicks and not just "is pressed"
    let mut mouse_pressed = false;

    // The main infinite "Input Update Draw" loop.
    loop {
        // ===========Input===========
        // Get the mouse position inside the game world.
        let mouse_position = relative_mouse_position(&main_camera);
        if is_key_down(KeyCode::Right) {}
        if is_key_down(KeyCode::Left) {}
        if is_key_down(KeyCode::Down) {}
        if is_key_down(KeyCode::Up) {}
        if is_mouse_button_down(MouseButton::Left) {
            if mouse_pressed == false {
                let pos = relative_mouse_position(&main_camera);
                debug!("Mouse click at relative x:{} , y:{}", pos.x(), pos.y())
            }
            mouse_pressed = true;
        } else {
            mouse_pressed = false;
        }

        // ===========Update===========
        // Checks for input related to camera and changes it accordingly.
        move_camera(&mut main_camera);

        // Run initialized systems schedule of legion ECS.
        schedule.execute(&mut world, &mut resources);

        // ===========Draw===========
        // Fill the canvas with white.
        clear_background(Color([255, 255, 255, 255]));

        // --- Camera space, render game objects.
        set_camera(Camera2D {
            target: main_camera.target,
            zoom: main_camera.zoom,
            ..Default::default()
        });

        // Draw the map.
        atlas.draw_tile(Tiles::Wall, vec2(0., 0.));

        // Draw the mouse cursor.
        draw_circle(
            mouse_position.x(),
            mouse_position.y(),
            0.2,
            Color([200, 150, 225, 255]),
        );

        // --- Fixed screen space, render ui.
        set_default_camera();
        draw_ui();

        next_frame().await
    }
}
