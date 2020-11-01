use crate::map::tiles::{Tile, Tiles};
use crate::map::{position_to_index, Point, Rect};
use macroquad::vec2;
use rand::prelude::*;
use std::cmp::{max, min};

/// Generates a map. Randomly placed walls, with screen edges.
pub fn _random_map(width: i32, height: i32) -> Vec<Tile> {
    let mut map = Vec::with_capacity((width * height) as usize);
    for x in 0..width {
        for y in 0..height {
            map.push(Tile {
                pos: vec2(x as f32, y as f32),
                kind: Tiles::Grass,
            })
        }
    }

    // Make the boundaries walls
    for x in 0..width {
        if let Some(tile) = map.get_mut(position_to_index(x, 0, width)) {
            tile.kind = Tiles::Wall;
        }
        if let Some(tile) = map.get_mut(position_to_index(x, height - 1, width)) {
            tile.kind = Tiles::Wall;
        }
    }
    for y in 0..height {
        if let Some(tile) = map.get_mut(position_to_index(0, y, width)) {
            tile.kind = Tiles::Wall;
        }
        if let Some(tile) = map.get_mut(position_to_index(width - 1, y, width)) {
            tile.kind = Tiles::Wall;
        }
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = rand::thread_rng();

    for _i in 0..200 {
        let x = rng.gen_range(0, width - 1);
        let y = rng.gen_range(0, height - 1);
        let index = position_to_index(x, y, width);
        if let Some(tile) = map.get_mut(index) {
            tile.kind = Tiles::Wall;
        }
    }

    map
}

/*
/// Generates a map. Randomly placed walls, with screen edges.
pub fn rooms_map(width: i32, height: i32) -> Vec<Tiles> {
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 3;
    const MAX_SIZE: i32 = 10;

    let mut map = vec![Tiles::Wall; (width * height) as usize];

    let mut rooms: Vec<Rect> = Vec::new();

    let mut rng = rand::thread_rng();

    for _ in 0..MAX_ROOMS {
        let room_width = rng.gen_range(MIN_SIZE, MAX_SIZE);
        let room_height = rng.gen_range(MIN_SIZE, MAX_SIZE);
        let x = rng.gen_range(0, height - room_height - 1) - 1;
        let y = rng.gen_range(0, width - room_width - 1) - 1;
        let new_room = Rect::new(Point { x, y }, room_width, room_height);

        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            rooms.push(new_room);
        }
    }

    for room in &rooms {
        apply_room_to_map(room, &mut map);
        let other_room = rooms.get(rng.gen_range(0, rooms.len())).unwrap();
        connect_rooms(room, other_room, &mut map);
    }

    map
}

/// Fill the provided rectangle with Grass tiles and place it on the map.
fn apply_room_to_map(room: &Rect, map: &mut [Tiles]) {
    for y in (room.top_left.y + 1)..=room.down_right.y {
        for x in (room.top_left.x + 1)..=room.down_right.x {
            if let Some(tile) = map.get_mut(position_to_index(x, y)) {
                *tile = Tiles::Grass;
            }
        }
    }
}

fn apply_vertical_corridor(starting_point: Point, len: i32, map: &mut [Tiles]) {
    let Point { x, y } = starting_point;
    for target_y in min(y, y + len)..=max(y, y + len) {
        let index = position_to_index(x, target_y);

        if let Some(tile) = map.get_mut(index) {
            *tile = Tiles::Grass;
        }
    }
}

fn apply_horizontal_corridor(starting_point: Point, len: i32, map: &mut [Tiles]) {
    let Point { x, y } = starting_point;
    for target_x in min(x, x + len)..=max(x, x + len) {
        let index = position_to_index(target_x, y);

        if let Some(tile) = map.get_mut(index) {
            *tile = Tiles::Grass;
        }
    }
}

/// Connect a pair of rooms with L shaped corridor.
fn connect_rooms(room1: &Rect, room2: &Rect, map: &mut [Tiles]) {
    let center1 = room1.center();
    let center2 = room2.center();
    apply_horizontal_corridor(center1, center2.x - center1.x, map);
    apply_vertical_corridor(center2, center1.y - center2.y, map)
}
*/
