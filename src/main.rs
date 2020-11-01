#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::clone_on_ref_ptr,
    clippy::else_if_without_else,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
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
    clippy::future_not_send,
    clippy::explicit_iter_loop
)]

use legion::{system, IntoQuery, Resources, Schedule, World};
use macroquad::{
    clear_background, debug, draw_circle, draw_text, is_key_pressed, is_mouse_button_down,
    load_texture, next_frame, set_camera, set_default_camera, vec2, warn, Camera2D, Color, KeyCode,
    MouseButton,
};

mod map;
use crate::map::generators::rooms_map;
use crate::map::tiles::{Tile, TileAtlas};

mod camera;
use crate::camera::{relative_mouse_position, Camera};

mod characters;
use crate::characters::player::Player;

mod utils;
use utils::settings::Settings;

#[macroquad::main("kiriRoguelike")]
async fn main() {
    // Load settings file.
    let settings = Settings::init("Settings.config");

    // Init world and resources of legion ECS.
    let mut world = World::default();
    let mut resources = Resources::default();

    // Construct a systems schedule of legion ECS.
    let mut schedule = Schedule::builder()
        .add_system(draw_map_system())
        .add_system(draw_player_system())
        .build();

    // Load assets.
    let texture = load_texture("assets/Tiles.png").await;

    // Construct TileAtlas.
    let atlas = TileAtlas::new(texture, 32., 32.);
    resources.insert(atlas);

    // Initialize main camera.
    let mut main_camera = Camera::default();

    // We need to save the state of the mouse button
    // to detect mouse clicks and not just "is pressed"
    let mut left_mouse_pressed = false;

    // Tiles is an enum of tile types, like Wall, Grass, Pengu.
    // Tile is a concrete struct with associated map coordinates.
    // `rooms_map()` is a generator that provides a layout. (There are
    // different types of generators)
    println!(
        "generating the map with {} by {} size",
        settings.width, settings.height
    );
    let map = rooms_map(settings.width, settings.height);
    // We push that map into the world, to draw it with `draw_map_system()`
    world.push((map,));

    // Create a player and insert them into the world.
    let bacing = Player::default();
    world.push((bacing,));

    // The main infinite "Input Update Draw" loop.
    loop {
        // ===========Input===========
        // Get the mouse position inside the game world.
        let mouse_position = relative_mouse_position(&main_camera);

        // Player input.
        left_mouse_pressed = handle_mouse(left_mouse_pressed, main_camera);
        handle_keyboard(&mut world);

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

        // First draw the map,
        // Then draw the player.
        schedule.execute(&mut world, &mut resources);

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

/// Go through the map and drow the tiles with provided TileAtlas.
#[system(for_each)]
fn draw_map(tiles: &Vec<Tile>, #[resource] atlas: &TileAtlas) {
    for tile in tiles {
        atlas.draw_tile(tile.kind, tile.pos);
    }
}

/// Draws the player. Shoul be called after the `draw_map`.
#[system(for_each)]
fn draw_player(player: &Player, #[resource] atlas: &TileAtlas) {
    let tile = &player.tile;
    atlas.draw_tile(tile.kind, tile.pos);
}

/// Render the fixed screen ui. (after `set_default_camera()`)
fn draw_ui() {
    let text_color: Color = Color([100, 100, 100, 150]);
    draw_text(",aoe to move camera", 10.0, 0.0, 20.0, text_color);
    draw_text("'. to zoom camera", 10.0, 30.0, 20.0, text_color);
    draw_text(
        "arrow keys to move the player",
        10.0,
        60.0,
        20.0,
        text_color,
    );
}

/// Handle the keyboard. Move the player.
fn handle_keyboard(world: &mut World) {
    let mut query = <(&mut Player,)>::query();

    for (bacing,) in query.iter_mut(world) {
        if is_key_pressed(KeyCode::Right) {
            bacing.walk(vec2(1., 0.));
        }
        if is_key_pressed(KeyCode::Left) {
            bacing.walk(vec2(-1., 0.));
        }
        if is_key_pressed(KeyCode::Down) {
            bacing.walk(vec2(0., -1.));
        }
        if is_key_pressed(KeyCode::Up) {
            bacing.walk(vec2(0., 1.));
        }
    }
}

/// Handle the mouse. Print the click position.
fn handle_mouse(left_mouse_pressed: bool, main_camera: Camera) -> bool {
    if is_mouse_button_down(MouseButton::Left) {
        if !left_mouse_pressed {
            let pos = relative_mouse_position(&main_camera);

            debug!("Mouse click at relative x:{} , y:{}", pos.x(), pos.y());
        }
        true
    } else {
        false
    }
}
