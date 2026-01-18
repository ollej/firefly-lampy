use crate::constants::{TILE_HEIGHT, TILE_WIDTH};
use firefly_rust::Point;

#[derive(Clone, Copy)]
pub struct Tile {
    pub position: Point,
    pub sprite_index: i32,
    pub solid: bool,
}

impl Tile {
    pub fn new(grid_x: i32, grid_y: i32, sprite_index: i32, is_solid: bool) -> Self {
        Self {
            position: Point {
                x: grid_x * TILE_WIDTH,
                y: grid_y * TILE_HEIGHT,
            },
            sprite_index,
            solid: is_solid,
        }
    }

    pub fn is_solid(&self) -> bool {
        self.solid
    }
}
