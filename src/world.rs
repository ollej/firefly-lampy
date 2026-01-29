use alloc::vec::Vec;

use firefly_rust::Point;

use crate::{
    camera::Camera,
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_HEIGHT, TILE_WIDTH, WORLD_HEIGHT, WORLD_WIDTH},
    drawing::draw_tile,
    rectangle::Rectangle,
    tile::Tile,
    utility::random_range,
};

pub struct World {
    tiles: Vec<Tile>,
    width: i32,
    height: i32,
}

impl World {
    pub fn rect() -> Rectangle {
        Rectangle {
            point: Point::new(0, 0),
            width: WORLD_WIDTH,
            height: WORLD_HEIGHT,
        }
    }

    pub fn new_from_2d_array(data: &[&[i32]]) -> Self {
        let height = data.len() as i32;
        let width = data.first().map_or(0, |row| row.len() as i32);

        let mut tiles = Vec::with_capacity((width * height) as usize);

        for (y, row) in data.iter().enumerate() {
            for (x, &sprite_index) in row.iter().enumerate() {
                let mut solid = true;
                if sprite_index == 0
                    || sprite_index == 16
                    || sprite_index == 17
                    || sprite_index == 24
                    || sprite_index == 25
                {
                    solid = false
                }

                tiles.push(Tile::new(x as i32, y as i32, sprite_index, solid));
            }
        }

        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn draw(&self, camera: &Camera) {
        let screen_start = camera.screen_to_world(Point { x: 0, y: 0 });
        let screen_end = camera.screen_to_world(Point {
            x: SCREEN_WIDTH,
            y: SCREEN_HEIGHT,
        });

        let start_x = (screen_start.x / TILE_WIDTH).max(0);
        let start_y = (screen_start.y / TILE_HEIGHT).max(0);
        let end_x = ((screen_end.x / TILE_WIDTH) + 1).min(self.width);
        let end_y = ((screen_end.y / TILE_HEIGHT) + 1).min(self.height);

        // Culling out-of-bounds tiles
        for y in start_y..end_y {
            for x in start_x..end_x {
                if let Some(tile) = self.get_tile(x, y)
                    && tile.sprite_index != 0
                {
                    let screen_pos = camera.world_to_screen(tile.position);
                    draw_tile(tile.sprite_index, screen_pos);
                }
            }
        }
    }

    pub fn draw_all_without_camera(&self) {
        // For testing / debug
        for tile in self.tiles.iter() {
            draw_tile(tile.sprite_index, tile.position);
        }
    }

    pub fn random_unblocked_point(&self) -> Point {
        self.random_unblocked_point_in_rectangle(Self::rect())
    }

    pub fn random_unblocked_point_in_rectangle(&self, rect: Rectangle) -> Point {
        let bottom_right = rect.bottom_right();
        let point = Point {
            x: random_range(rect.x() as u32, bottom_right.x as u32) as i32,
            y: random_range(rect.y() as u32, bottom_right.y as u32) as i32,
        };

        if self.is_blocked(point) {
            self.random_unblocked_point_in_rectangle(rect)
        } else {
            point
        }
    }

    pub fn is_blocked(&self, point: Point) -> bool {
        let tile_x = point.x / TILE_WIDTH;
        let tile_y = point.y / TILE_HEIGHT;
        self.get_tile(tile_x, tile_y)
            .map(|t| t.is_solid())
            .unwrap_or(false)
    }

    pub fn is_in_goal(&self, point: Point) -> bool {
        let tile_x = point.x / TILE_WIDTH;
        let tile_y = point.y / TILE_HEIGHT;
        self.get_tile(tile_x, tile_y)
            .map(|t| t.is_goal())
            .unwrap_or(false)
    }

    fn get_tile(&self, x: i32, y: i32) -> Option<&Tile> {
        let index = self.convert_pos_to_index(x, y)?;
        self.tiles.get(index)
    }

    fn convert_pos_to_index(&self, x: i32, y: i32) -> Option<usize> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            Some((y * self.width + x) as usize)
        } else {
            None
        }
    }
}
