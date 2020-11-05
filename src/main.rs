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

use legion::{system, Resources, Schedule, World};

use macroquad::{
    clear_background, debug, draw_circle, draw_text, error, is_key_pressed, is_mouse_button_down,
    load_texture, next_frame, set_camera, set_default_camera, warn, Camera2D, Color, KeyCode,
    MouseButton, Vec2, BLACK, GRAY, WHITE,
};

mod map;
use crate::map::generators::{_random_map, rooms_map};
use crate::map::tiles::{Position, Tile, TileAtlas};
use crate::map::Rect;

mod characters;
use crate::characters::player::{IsPlayer, Viewshed};

mod utils;
use utils::settings::Settings;
use utils::{camera, camera::relative_mouse_position, camera::Camera};

use std::cmp::max;
#[macroquad::main("kiriRoguelike")]
async fn main() {
    // Load settings file.
    let settings = Settings::init("Settings.config");

    // Init world and resources of legion ECS.
    let mut world = World::default();
    let mut resources = Resources::default();
    let mut schedule = Schedule::builder()
        .add_system(update_viewshed_system())
        .add_system(handle_keyboard_system())
        .add_system(draw_system())
        .build();

    // Load assets.
    let texture = load_texture("assets/Tiles.png").await;

    // Construct TileAtlas.
    let atlas = TileAtlas::new(texture, 32., 32.);
    resources.insert(atlas);
    // We need to save the state of the mouse button
    // to detect mouse clicks and not just "is pressed"
    let mut left_mouse_pressed = false;

    // Tile is an enum of tile types, like Wall, Grass, Pengu.
    // `rooms_map()` is a generator for the level. (There are
    // different types of generators)
    println!(
        "generating the map {}:{} size",
        settings.width, settings.height
    );
    let map = rooms_map(settings.width, settings.height, settings.gen_param);
    // We push that map into the world, to draw it with `draw_system()`
    resources.insert(map.tiles);
    resources.insert(map.revealed_tiles);

    // Starting position is the center of the last room.
    let starting_position = Position::from(
        map.rooms
            .unwrap_or(Vec::new())
            .last()
            .unwrap_or(&Rect::default())
            .center(),
    );
    // Insert the player into the world.
    world.push((
        Tile::Pengu,
        starting_position,
        IsPlayer {},
        Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        },
    ));

    // Initialize main camera.
    let mut main_camera = Camera::default();
    main_camera.set_target(starting_position.into());
    // The infinite game loop.
    loop {
        // ===========Input===========
        // Get the mouse position inside the game world.
        let mouse_position = relative_mouse_position(&main_camera);
        left_mouse_pressed = handle_mouse(left_mouse_pressed, mouse_position);

        // ===========Update===========
        // Checks for input related to camera and changes it accordingly.
        camera::scroll(&mut main_camera, settings.scroll_speed, settings.zoom_speed);

        // ===========Draw===========
        // Fill the canvas with white.
        clear_background(BLACK);

        // --- Camera space, render game objects.
        let (target, zoom) = main_camera.get();
        set_camera(Camera2D {
            target,
            zoom,
            ..macroquad::Camera2D::default()
        });

        // ----------ECS schedule exec---------------
        schedule.execute(&mut world, &mut resources);

        // Draw the mouse cursor.
        draw_circle(
            mouse_position.x(),
            mouse_position.y(),
            0.1,
            Color([100, 75, 120, 255]),
        );

        // --- Fixed screen space, render ui.
        set_default_camera();
        draw_ui();

        next_frame().await
    }
}

/*
fn check_moves(world: &mut World) {
    let mut query = <(&Position, &IsWalkable)>::query();
    let (left, mut right) = world.split_for_query(&query);
    let mut mover_query = <&mut Mover>::query();
    for mover in mover_query.iter_mut(&mut right) {
        for (pos, is_walkable) in query.iter(&left) {
            if is_walkable.get() {
                if pos == mover {
                    mover.able_to_move = true;
                }
            }
        }
    }
}
*/

/// Calculate the viewshed.
#[system(for_each)]
fn update_viewshed(
    viewshed: &mut Viewshed,
    origin: &Position,
    _: &IsPlayer,
    #[resource] map: &Vec<Vec<Tile>>,
    #[resource] revealed_tiles: &mut Vec<Vec<bool>>,
) {
    use symmetric_shadowcasting::compute_fov;
    if viewshed.dirty {
        viewshed.visible_tiles.clear();
        let mut is_blocking = |pos: (isize, isize)| {
            let outside = (pos.1 as usize) >= map.len() || (pos.0 as usize) >= map[0].len();
            return outside || map[pos.0 as usize][pos.1 as usize].is_opaque();
        };

        let mut mark_visible = |pos: (isize, isize)| {
            let outside = (pos.1 as usize) >= map.len() || (pos.0 as usize) >= map[0].len();
            let in_range = (pos.1 as usize) >= max(origin.y - viewshed.range, 0) as usize
                && (pos.1 as usize) <= (origin.y + viewshed.range) as usize
                && (pos.0 as usize) >= max(origin.x - viewshed.range, 0) as usize
                && (pos.0 as usize) <= (origin.x + viewshed.range) as usize;
            let tile_pos = Position {
                x: pos.0 as i32,
                y: pos.1 as i32,
            };
            if in_range && !outside && !viewshed.visible_tiles.contains(&tile_pos) {
                viewshed.visible_tiles.push(tile_pos);
                revealed_tiles[tile_pos.x as usize][tile_pos.y as usize] = true;
            }
        };

        let (ox, oy) = origin.as_tuple();
        let origin = (ox as isize, oy as isize);
        compute_fov(origin, &mut is_blocking, &mut mark_visible);
    }
}

// Render the map and then the in-game entities.
#[system(for_each)]
fn draw(
    pos: &mut Position,
    tile: &Tile,
    _: &IsPlayer,
    viewshed: &Viewshed,
    #[resource] map: &Vec<Vec<Tile>>,
    #[resource] revealed_tiles: &Vec<Vec<bool>>,
    #[resource] atlas: &TileAtlas,
) {
    for (x, row) in map.iter().enumerate() {
        for (y, map_tile) in row.iter().enumerate() {
            if revealed_tiles[x][y] {
                if viewshed.visible_tiles.contains(&Position {
                    x: x as i32,
                    y: y as i32,
                }) {
                    atlas.draw_tile(
                        map_tile,
                        &Position {
                            x: x as i32,
                            y: y as i32,
                        },
                        WHITE,
                    );
                } else {
                    atlas.draw_tile(
                        map_tile,
                        &Position {
                            x: x as i32,
                            y: y as i32,
                        },
                        GRAY,
                    );
                }
            }
        }
    }
    atlas.draw_tile(tile, pos, WHITE);
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

/// Handle the keyboard. Try to move the player (handles collisions).
#[system(for_each)]
fn handle_keyboard(
    current_pos: &mut Position,
    viewshed: &mut Viewshed,
    _: &IsPlayer,
    #[resource] map: &Vec<Vec<Tile>>,
) {
    // Saves the current position in case the destination is not walkable.
    let mut pos = current_pos.clone();
    if is_key_pressed(KeyCode::Right) {
        pos.x += 1;
    }
    if is_key_pressed(KeyCode::Left) {
        pos.x -= 1;
    }
    if is_key_pressed(KeyCode::Down) {
        pos.y -= 1;
    }
    if is_key_pressed(KeyCode::Up) {
        pos.y += 1;
    }

    // Resets the position if the destination is not walkable.
    // Prints coords of out-of-bounds entities.
    if let Some(row) = map.get(pos.x as usize) {
        if let Some(tile) = row.get(pos.y as usize) {
            if tile.is_walkable() {
                current_pos.x = pos.x;
                current_pos.y = pos.y;
                viewshed.dirty = true;
            }
        }
    }
}

/// Handle the mouse. Print the click position.
fn handle_mouse(left_mouse_pressed: bool, mouse_position: Vec2) -> bool {
    if is_mouse_button_down(MouseButton::Left) {
        if !left_mouse_pressed {
            debug!(
                "Mouse click at relative x:{} , y:{}",
                mouse_position.x() as i32,
                mouse_position.y() as i32
            );
        }
        true
    } else {
        false
    }
}
