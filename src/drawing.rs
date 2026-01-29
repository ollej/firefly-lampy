use firefly_rust::{draw_text, Color, Point, HEIGHT, WIDTH};

use crate::{constants::*, state::*};

pub fn display_large_text_color(text: &str, position: Point, color: Color) {
    let state = get_state();
    let font = state.font_large.as_font();
    draw_text(text, &font, position, color);
}

pub fn display_small_text_color(text: &str, position: Point, color: Color) {
    let state = get_state();
    let font = state.font_small.as_font();
    draw_text(text, &font, position, color);
}

pub fn display_centered_message(color: Option<Color>, lines: &[&str]) {
    let color = color.unwrap_or(Color::Black);
    let y_pos: i32 = HEIGHT / 2 + FONT_LARGE_HEIGHT - lines.len() as i32 * FONT_LARGE_HEIGHT / 2;
    for (i, line) in lines.iter().enumerate() {
        display_large_text_color(
            line,
            Point {
                x: WIDTH / 2 - (line.len() as i32 * FONT_LARGE_HALF_WIDTH),
                y: y_pos + i as i32 * FONT_LARGE_HEIGHT,
            },
            color,
        );
    }
}

pub fn display_large_left_message(lines: &[&str], color: Color) {
    let y_pos: i32 = FONT_LARGE_HEIGHT + 4;
    for (i, line) in lines.iter().enumerate() {
        display_large_text_color(
            line,
            Point {
                x: FONT_LARGE_HALF_WIDTH,
                y: y_pos + i as i32 * FONT_LARGE_HEIGHT,
            },
            color,
        );
    }
}

pub fn display_small_left_message(lines: &[&str], position: Point, color: Color) {
    let y_pos: i32 = position.y + FONT_SMALL_HEIGHT;
    for (i, line) in lines.iter().enumerate() {
        display_small_text_color(
            line,
            Point {
                x: position.x,
                y: y_pos + i as i32 * FONT_SMALL_HEIGHT,
            },
            color,
        );
    }
}
