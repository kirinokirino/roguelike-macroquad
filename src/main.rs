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
    clippy::rest_pat_in_fully_bound_structs,
    clippy::unwrap_used,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::string_add,
    clippy::wildcard_enum_match_arm,
    clippy::wrong_pub_self_convention
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::unknown_clippy_lints,
    clippy::expect_used,
    clippy::future_not_send
)]

use legion::{Resources, Schedule, World};
use macroquad::{
    clear_background, debug, draw_circle, draw_text, is_key_down, is_mouse_button_down,
    load_texture, next_frame, set_camera, set_default_camera, vec2, warn, Camera2D, Color, KeyCode,
    MouseButton,
};

mod map;
use crate::map::tiles::{TileAtlas, Tiles};

mod camera;
use crate::camera::{relative_mouse_position, Camera};

#[macroquad::main("kiriRoguelike")]
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

    // Initialize main camera.
    let mut main_camera = Camera::default();

    // We need to save the state of the mouse button
    // to detect mouse clicks and not just "is pressed"
    let mut left_mouse_pressed = false;

    // The main infinite "Input Update Draw" loop.
    loop {
        // ===========Input===========
        // Get the mouse position inside the game world.
        let mouse_position = relative_mouse_position(&main_camera);

        // Player input.
        left_mouse_pressed = handle_mouse(left_mouse_pressed, main_camera);
        handle_keyboard();

        // ===========Update===========
        // Checks for input related to camera and changes it accordingly.
        camera::scroll(&mut main_camera);

        // Run initialized systems schedule of legion ECS.
        schedule.execute(&mut world, &mut resources);

        // ===========Draw===========
        // Fill the canvas with white.
        clear_background(Color([255, 255, 255, 255]));

        // --- Camera space, render game objects.
        let (target, zoom) = main_camera.get();
        set_camera(Camera2D {
            target,
            zoom,
            ..macroquad::Camera2D::default()
        });

        // Draw the map.
        atlas.draw_tile(&Tiles::Wall, vec2(0., 0.));

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

/// Render the fixed screen ui. (after `set_default_camera()`)
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

fn handle_keyboard() {
    if is_key_down(KeyCode::Right) {}
    if is_key_down(KeyCode::Left) {}
    if is_key_down(KeyCode::Down) {}
    if is_key_down(KeyCode::Up) {}
}

fn handle_mouse(left_mouse_pressed: bool, main_camera: Camera) -> bool {
    if is_mouse_button_down(MouseButton::Left) {
        if !left_mouse_pressed {
            let pos = relative_mouse_position(&main_camera);
            debug!("Mouse click at relative x:{} , y:{}", pos.x(), pos.y())
        }
        true
    } else {
        false
    }
}
