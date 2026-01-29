use alloc::vec::Vec;

use firefly_rust::{
    clear_screen, draw_sub_image, set_canvas, unset_canvas, CanvasBuf, Color, Point, Size,
};

use crate::{
    camera::Camera,
    constants::{
        SCREEN_HEIGHT, SCREEN_WIDTH, SPRITES_H, SPRITES_W, TILE_HEIGHT, TILE_WIDTH, WORLD_HEIGHT,
        WORLD_WIDTH,
    },
    rectangle::Rectangle,
    state::get_state,
    tile::Tile,
    utility::random_range,
};

pub type Sprite = i32;

pub struct World {
    canvas: CanvasBuf,
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

        let canvas = Self::draw_tiles_to_canvas(&tiles, width, height);

        Self {
            canvas,
            tiles,
            width,
            height,
        }
    }

    pub fn draw_tile(sprite: Sprite, point: Point) {
        let state = get_state();
        let tile_sprite = state.spritesheet.as_image().sub(
            Point {
                x: ((sprite % 8) * TILE_WIDTH),
                y: ((sprite / 8) * TILE_HEIGHT),
            },
            Size {
                width: SPRITES_W,
                height: SPRITES_H,
            },
        );
        draw_sub_image(&tile_sprite, point);
    }

    pub fn draw(&self, camera: &Camera) {
        let screen_start = camera.screen_to_world(Point::MIN);
        let sub_image = self.canvas.as_image().sub(screen_start, Size::MAX);
        draw_sub_image(&sub_image, Point::MIN)
    }

    pub fn draw_tiles_to_canvas(tiles: &Vec<Tile>, width: i32, height: i32) -> CanvasBuf {
        let canvas_buf = CanvasBuf::new(Size::new(WORLD_WIDTH, WORLD_HEIGHT));
        let canvas = &canvas_buf.as_canvas();
        set_canvas(canvas);
        clear_screen(Color::White);

        for y in 0..width {
            for x in 0..height {
                if let Some(index) = Self::convert_pos_to_index(x, y, width, height) {
                    if let Some(tile) = tiles.get(index) {
                        Self::draw_tile(tile.sprite_index, tile.position);
                    }
                }
            }
        }

        unset_canvas();

        canvas_buf
    }

    pub fn draw_tiles_to_camera(&self, camera: &Camera) {
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
                    Self::draw_tile(tile.sprite_index, screen_pos);
                }
            }
        }
    }

    pub fn draw_all_without_camera(&self) {
        // For testing / debug
        for tile in self.tiles.iter() {
            Self::draw_tile(tile.sprite_index, tile.position);
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
        let index = Self::convert_pos_to_index(x, y, self.width, self.height)?;
        self.tiles.get(index)
    }

    fn convert_pos_to_index(x: i32, y: i32, width: i32, height: i32) -> Option<usize> {
        if x >= 0 && x < width && y >= 0 && y < height {
            Some((y * width + x) as usize)
        } else {
            None
        }
    }
}
