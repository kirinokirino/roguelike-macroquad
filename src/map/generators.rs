use crate::map::tiles::Tile;
use crate::map::{Point, Rect};
use fastrand::Rng;
use std::iter::repeat_with;

use std::cmp::{max, min};
/// Generates a map. Randomly placed walls.
pub fn _random_map(width: usize, height: usize, num_walls: i32) -> Vec<Vec<Tile>> {
    let mut map = create_grass_map(width, height);
    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let rng = Rng::new();

    for _i in 0..num_walls {
        let x = rng.usize(..(width - 1) as usize);
        let y = rng.usize(..(height - 1) as usize);
        map[x][y] = Tile::Wall;
    }

    make_borders(width, height, &mut map);
    map
}

/// Creates a map filled with grass.
fn create_grass_map(width: usize, height: usize) -> Vec<Vec<Tile>> {
    vec![vec![Tile::Grass; height]; width]
}

/// Creates the border of walls for the provided map.
fn make_borders(width: usize, height: usize, map: &mut Vec<Vec<Tile>>) {
    // Make the boundaries walls
    for x in 0..width - 1 {
        map[x as usize][0] = Tile::Wall;
        map[x as usize][(height - 1) as usize] = Tile::Wall;
    }
    for y in 0..height - 1 {
        map[0][y as usize] = Tile::Wall;
        map[(width - 1) as usize][y as usize] = Tile::Wall;
    }
}

/// Generates a map. Randomly placed broken rooms.
pub fn rooms_map(width: usize, height: usize, max_rooms: i32) -> Vec<Vec<Tile>> {
    const MIN_SIZE: usize = 3;
    const MAX_SIZE: usize = 10;

    let mut map = create_grass_map(width, height);

    let mut rooms: Vec<Rect> = Vec::new();
    let rng = Rng::new();

    for _ in 0..max_rooms {
        let room_width = rng.usize(MIN_SIZE..MAX_SIZE);
        let room_height = rng.usize(MIN_SIZE..MAX_SIZE);
        let x = rng.usize(..(width - room_width));
        let y = rng.usize(..(height - room_height));
        let new_room = Rect::new(
            Point {
                x: x as i32,
                y: y as i32,
            },
            room_width as i32,
            room_height as i32,
        );

        /*
        let mut ok = true;
        for other_room in rooms.iter() {
            if new_room.intersect(other_room) {
                ok = false
            }
        }
        if ok {
            rooms.push(new_room);
        }
        */
        rooms.push(new_room);
    }

    for room in &rooms {
        apply_room_to_map(room, &mut map);
    }
    let other_rooms: Vec<&Rect> = repeat_with(|| rooms.get(rng.usize(..rooms.len())).unwrap())
        .take(max_rooms as usize)
        .collect();
    for (room, other_room) in rooms.iter().zip(other_rooms) {
        connect_rooms(room, other_room, &mut map);
    }
    make_borders(width, height, &mut map);
    map
}

/// Fill the provided rectangle with Grass tiles and place it on the map.
fn apply_room_to_map(room: &Rect, map: &mut Vec<Vec<Tile>>) {
    for y in room.top_left.y..=room.down_right.y {
        for x in room.top_left.x..=room.down_right.x {
            map[x as usize][y as usize] = Tile::Wall;
        }
    }
    for x in room.top_left.x + 1..room.down_right.x {
        for y in room.top_left.y + 1..room.down_right.y {
            map[x as usize][y as usize] = Tile::Grass;
        }
    }
}

fn apply_vertical_corridor(starting_point: Point, len: i32, map: &mut Vec<Vec<Tile>>) {
    let Point { x, y } = starting_point;
    for target_y in min(y, y + len)..=max(y, y + len) {
        map[x as usize][target_y as usize] = Tile::Grass;
    }
}

fn apply_horizontal_corridor(starting_point: Point, len: i32, map: &mut Vec<Vec<Tile>>) {
    let Point { x, y } = starting_point;
    for target_x in min(x, x + len)..=max(x, x + len) {
        map[target_x as usize][y as usize] = Tile::Grass;
    }
}

/// Connect a pair of rooms with L shaped corridor.
fn connect_rooms(room1: &Rect, room2: &Rect, map: &mut Vec<Vec<Tile>>) {
    let center1 = room1.center();
    let center2 = room2.center();
    apply_horizontal_corridor(center1, center2.x - center1.x, map);
    apply_vertical_corridor(center2, center1.y - center2.y, map)
}
