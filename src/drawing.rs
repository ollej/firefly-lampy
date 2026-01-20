use firefly_rust::{draw_sub_image, draw_text, Color, Point, Size, HEIGHT, WIDTH};

use crate::{constants::*, state::*};

pub type Sprite = i32;

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

pub fn display_text_color(text: &str, position: Point, color: Color) {
    let state = get_state();
    let font = state.font.as_font();
    draw_text(text, &font, position, color);
}

pub fn display_centered_message(color: Option<Color>, lines: &[&str]) {
    let color = color.unwrap_or(Color::Black);
    let y_pos: i32 = HEIGHT / 2 + FONT_BASE_LINE - lines.len() as i32 * LINE_HEIGHT / 2;
    for (i, line) in lines.iter().enumerate() {
        display_text_color(
            line,
            Point {
                x: WIDTH / 2 - (line.len() as i32 * HALF_FONT_WIDTH),
                y: y_pos + i as i32 * LINE_HEIGHT,
            },
            color,
        );
    }
}

pub fn display_left_message(lines: &[&str], color: Color) {
    let y_pos: i32 = FONT_BASE_LINE + 4;
    for (i, line) in lines.iter().enumerate() {
        display_text_color(
            line,
            Point {
                x: 4,
                y: y_pos + i as i32 * LINE_HEIGHT,
            },
            color,
        );
    }
}
