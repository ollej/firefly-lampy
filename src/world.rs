use crate::camera::*;
use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, TILE_HEIGHT, TILE_WIDTH};
use crate::drawing::*;
use crate::tile::Tile;
use crate::utility::random_range;
use alloc::vec::Vec;
use firefly_rust::{Point, log_debug};

pub struct World {
    tiles: Vec<Tile>,
    width: i32,
    height: i32,
    pub pixel_width: i32,
    pub pixel_height: i32,
}

impl World {
    pub fn new(width: i32, height: i32) -> Self {
        let tiles_max = (width * height) as usize;
        let mut tiles = Vec::with_capacity(tiles_max);

        for y in 0..height {
            for x in 0..width {
                if y == 0 || y == height || x == 0 || x == width {
                    tiles.push(Tile::new(x, y, 0, false))
                } else {
                    tiles.push(Tile::new(x, y, 1, false))
                }
            }
        }

        Self {
            tiles,
            width,
            height,
            pixel_width: width * TILE_WIDTH,
            pixel_height: height * TILE_HEIGHT,
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
            pixel_width: width * TILE_WIDTH,
            pixel_height: height * TILE_HEIGHT,
        }
    }

    pub fn draw_all_without_camera(&self) {
        // For testing / debug
        for tile in self.tiles.iter() {
            draw_tile(tile.sprite_index, tile.position);
        }
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

    pub fn random_unblocked_point(&self) -> Point {
        let point = Point {
            x: random_range(0, (self.pixel_width - 1) as u32) as i32,
            y: random_range(0, (self.pixel_height - 1) as u32) as i32,
        };

        if self.is_blocked(point) {
            self.random_unblocked_point()
        } else {
            point
        }
    }

    pub fn is_blocked(&self, point: Point) -> bool {
        let tile_x = point.x / TILE_WIDTH;
        let tile_y = point.y / TILE_HEIGHT;
        //log_debug("checking blocks");
        let solid = self
            .get_tile(tile_x, tile_y)
            .map(|t| t.is_solid())
            .unwrap_or(false);
        //if solid {
        //    log_debug("solid");
        //}
        solid
    }

    pub fn is_in_goal(&self, point: Point) -> bool {
        let tile_x = point.x / TILE_WIDTH;
        let tile_y = point.y / TILE_HEIGHT;
        self.get_tile(tile_x, tile_y)
            .map(|t| t.is_goal())
            .unwrap_or(false)
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
                if let Some(tile) = self.get_tile(x, y) {
                    let screen_pos = camera.world_to_screen(tile.position);
                    draw_tile(tile.sprite_index, screen_pos);
                }
            }
        }
    }
}
