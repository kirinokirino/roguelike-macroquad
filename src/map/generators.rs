use crate::map::tiles::Tile;
use crate::map::{Point, Rect};
use fastrand::Rng;
use std::cmp::{max, min};

pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub revealed_tiles: Vec<Vec<bool>>,
    pub rooms: Option<Vec<Rect>>,
    width: usize,
    height: usize,
}

impl Map {
    /// Creates a new map filled with provided tile.
    pub fn new(fill_tile: Tile, width: usize, height: usize) -> Self {
        let tiles = vec![vec![fill_tile; height]; width];
        let revealed_tiles = vec![vec![false; height]; width];
        Self {
            tiles,
            revealed_tiles,
            rooms: None,
            width,
            height,
        }
    }

    pub fn set_rooms(&mut self, rooms: Option<Vec<Rect>>) {
        self.rooms = rooms;
        self.apply_rooms();
    }

    /// Creates the border of walls for the provided map.
    pub fn make_borders(&mut self) {
        // Make the boundaries walls
        for x in 0..self.width {
            self.tiles[x as usize][0] = Tile::Wall;
            self.tiles[x as usize][(self.height - 1) as usize] = Tile::Wall;
        }
        for y in 0..self.height {
            self.tiles[0][y as usize] = Tile::Wall;
            self.tiles[(self.width - 1) as usize][y as usize] = Tile::Wall;
        }
    }

    fn apply_vertical_corridor(&mut self, starting_point: &Point, len: i32) {
        let (x, y) = starting_point.as_tuple();
        for target_y in min(y, y + len)..=max(y, y + len) {
            self.tiles[x as usize][target_y as usize] = Tile::Grass;
        }
    }

    fn apply_horizontal_corridor(&mut self, starting_point: &Point, len: i32) {
        let (x, y) = starting_point.as_tuple();
        for target_x in min(x, x + len)..=max(x, x + len) {
            self.tiles[target_x as usize][y as usize] = Tile::Grass;
        }
    }

    fn apply_rooms(&mut self) {
        if let Some(rooms) = &self.rooms {
            for room in rooms.iter() {
                let walls = room.get_walls_positions();
                for (x, y) in walls {
                    self.tiles[x][y] = Tile::Wall;
                }

                let floors = room.get_floors_positions();
                for (x, y) in floors {
                    self.tiles[x][y] = Tile::Grass;
                }
            }
            self.connect_rooms();
        }
    }

    /// Connect a pair of rooms with L shaped corridor.
    fn connect_rooms(&mut self) {
        let mut corridors: Vec<(Point, Point)> = Vec::new();
        if let Some(rooms) = &self.rooms {
            for (room, next_room) in rooms.iter().zip(rooms.iter().skip(1)) {
                let center1 = room.center();
                let center2 = next_room.center();
                corridors.push((center1, center2));
            }
        }
        for (center1, center2) in corridors.iter() {
            self.apply_horizontal_corridor(center1, center2.x - center1.x);
            self.apply_vertical_corridor(center2, center1.y - center2.y);
        }
    }
}

/// Generates a map. Randomly placed broken rooms.
pub fn rooms_map(width: usize, height: usize, max_rooms: i32) -> Map {
    const MIN_SIZE: usize = 3;
    const MAX_SIZE: usize = 15;

    let mut map = Map::new(Tile::Wall, width, height);

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

    map.set_rooms(Some(rooms));

    /*
    let other_rooms: Vec<&Rect> = repeat_with(|| rooms.get(rng.usize(..rooms.len())).unwrap())
        .take(max_rooms as usize)
        .collect();
    for (room, other_room) in rooms.iter().zip(other_rooms) {
        connect_rooms(room, other_room, &mut map);
    }
    */
    map.make_borders();
    map
}

/// Generates a map. Randomly placed walls.
pub fn _random_map(width: usize, height: usize, num_walls: i32) -> Map {
    let mut map = Map::new(Tile::Grass, width, height);
    // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
    // First, obtain the thread-local RNG:
    let rng = Rng::new();

    for _i in 0..num_walls {
        let x = rng.usize(..(width - 1) as usize);
        let y = rng.usize(..(height - 1) as usize);
        map.tiles[x][y] = Tile::Wall;
    }

    map.make_borders();
    map
}
