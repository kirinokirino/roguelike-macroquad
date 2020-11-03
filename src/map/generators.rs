use crate::map::tiles::Tile;
use fastrand::Rng;

/// Generates a map. Randomly placed walls, with screen edges.
pub fn _random_map(width: usize, height: usize, num_walls: i32) -> Vec<Vec<Tile>> {
    let mut map = vec![vec![Tile::Grass; width]; height];
    for x in 0..width {
        for y in 0..height {
            map[x as usize][y as usize] = Tile::Grass;
        }
    }

    // Make the boundaries walls
    for x in 0..width {
        map[x as usize][0] = Tile::Wall;
        map[x as usize][(height - 1) as usize] = Tile::Wall;
    }
    for y in 0..height {
        map[0][y as usize] = Tile::Wall;
        map[(width - 1) as usize][y as usize] = Tile::Wall;
    }

    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let mut rng = Rng::new();

    for _i in 0..num_walls {
        let x = rng.usize(..(width - 1) as usize);
        let y = rng.usize(..(height - 1) as usize);
        map[x][y] = Tile::Wall;
    }

    map
}

/*
/// Generates a map. Randomly placed walls, with screen edges.
pub fn rooms_map(width: i32, height: i32) -> Vec<Tile> {
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 3;
    const MAX_SIZE: i32 = 10;

    let mut map = Vec::with_capacity((width * height) as usize);
    for x in 0..width {
        for y in 0..height {
            map.push(Tile::new(Tile::Wall, (x, y)))
        }
    }

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
        apply_room_to_map(room, &mut map, width);
        let other_room = rooms.get(rng.gen_range(0, rooms.len())).unwrap();
        connect_rooms(room, other_room, &mut map, width);
    }

    map
}

/// Fill the provided rectangle with Grass tiles and place it on the map.
fn apply_room_to_map(room: &Rect, map: &mut [Tile], width: i32) {
    for y in (room.top_left.y + 1)..=room.down_right.y {
        for x in (room.top_left.x + 1)..=room.down_right.x {
            if let Some(tile) = map.get_mut(position_to_index(x, y, width)) {
                tile.kind = Tile::Grass;
            }
        }
    }
}

fn apply_vertical_corridor(starting_point: Point, len: i32, map: &mut [Tile], width: i32) {
    let Point { x, y } = starting_point;
    for target_y in min(y, y + len)..=max(y, y + len) {
        let index = position_to_index(x, target_y, width);

        if let Some(tile) = map.get_mut(index) {
            tile.kind = Tile::Grass;
        }
    }
}

fn apply_horizontal_corridor(starting_point: Point, len: i32, map: &mut [Tile], width: i32) {
    let Point { x, y } = starting_point;
    for target_x in min(x, x + len)..=max(x, x + len) {
        let index = position_to_index(target_x, y, width);

        if let Some(tile) = map.get_mut(index) {
            tile.kind = Tile::Grass;
        }
    }
}

/// Connect a pair of rooms with L shaped corridor.
fn connect_rooms(room1: &Rect, room2: &Rect, map: &mut [Tile], width: i32) {
    let center1 = room1.center();
    let center2 = room2.center();
    apply_horizontal_corridor(center1, center2.x - center1.x, map, width);
    apply_vertical_corridor(center2, center1.y - center2.y, map, width)
}
*/
