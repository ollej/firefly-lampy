
use alloc::vec::Vec;
use crate::drawing::*;
use crate::tile::Tile;
use crate::utility::*;

const WORLD_HEIGHT : i32 = 800;
const WORLD_WIDTH : i32 = 800; 

pub struct World {
    tiles: Vec<Tile>,
    width: i32,
    height: i32,
}

impl World {
    pub fn new(width: i32, height: i32) -> Self {
        let tiles_max = (width * height) as usize;
        let mut tiles = Vec::with_capacity(tiles_max);

        for y in 0..height {
            for x in 0..width {
                if y == 0 || y == height || x == 0 || x == width {
                    tiles.push(Tile::new(x,y,0))
                } else {
                    tiles.push(Tile::new(x,y,1)) 
                }

                
            }
        }

        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn get_tile_type(grid_x: u8, grid_y: u8) {

    }

    pub fn draw_all_without_camera(&self) {
        for tile in self.tiles.iter() {
            draw_tile(tile.sprite_index, tile.position);
        }
    }
}