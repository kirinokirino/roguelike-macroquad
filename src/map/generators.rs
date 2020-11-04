use crate::map::tiles::Tile;
use crate::map::{Point, Rect};
use fastrand::Rng;
use std::iter::repeat_with;

use std::cmp::{max, min};
/// Generates a map. Randomly placed walls.
pub fn _random_map(width: usize, height: usize, num_walls: i32) -> Vec<Vec<Tile>> {
    let mut map = create_map(Tile::Grass, width, height);
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
fn create_map(fill_tile: Tile, width: usize, height: usize) -> Vec<Vec<Tile>> {
    vec![vec![fill_tile; height]; width]
}

/// Creates the border of walls for the provided map.
fn make_borders(width: usize, height: usize, map: &mut Vec<Vec<Tile>>) {
    // Make the boundaries walls
    for x in 0..width {
        map[x as usize][0] = Tile::Wall;
        map[x as usize][(height - 1) as usize] = Tile::Wall;
    }
    for y in 0..height {
        map[0][y as usize] = Tile::Wall;
        map[(width - 1) as usize][y as usize] = Tile::Wall;
    }
}

/// Generates a map. Randomly placed broken rooms.
pub fn rooms_map(width: usize, height: usize, max_rooms: i32) -> (Vec<Rect>, Vec<Vec<Tile>>) {
    const MIN_SIZE: usize = 3;
    const MAX_SIZE: usize = 15;

    let mut map = create_map(Tile::Wall, width, height);

    let mut rooms: Vec<Rect> = Vec::new();
    let rng = Rng::new();

    for _ in 0..max_rooms {
        let room_width = rng.usize(MIN_SIZE..MAX_SIZE);
        let room_height = rng.usize(MIN_SIZE..MAX_SIZE);
        let x = rng.usize(1..(width - room_width) - 1);
        let y = rng.usize(1..(height - room_height) - 1);
        let new_room = Rect::new(
            Point {
                x: x as i32,
                y: y as i32,
            },
            room_width as i32,
            room_height as i32,
        );

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

    for room in rooms.iter() {
        apply_room_to_map(room, &mut map);
    }

    for (room, next_room) in rooms.iter().zip(rooms.iter().skip(1)) {
        connect_rooms(room, next_room, &mut map);
    }

    /*
    let other_rooms: Vec<&Rect> = repeat_with(|| rooms.get(rng.usize(..rooms.len())).unwrap())
        .take(max_rooms as usize)
        .collect();
    for (room, other_room) in rooms.iter().zip(other_rooms) {
        connect_rooms(room, other_room, &mut map);
    }
    */
    make_borders(width, height, &mut map);
    (rooms, map)
}

/// Fill the provided rectangle with Grass tiles and place it on the map.
fn apply_room_to_map(room: &Rect, map: &mut Vec<Vec<Tile>>) {
    let walls = room.get_walls_positions();
    for (x, y) in walls {
        map[x][y] = Tile::Wall;
    }

    let floors = room.get_floors_positions();
    for (x, y) in floors {
        map[x][y] = Tile::Grass;
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
