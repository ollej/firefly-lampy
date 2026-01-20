use alloc::format;
use firefly_rust::{clear_screen, draw_image, Color, Point};

use crate::{constants::*, drawing::*, game_state::*, palette::*, state::*};

pub fn render_title() {
    let state = get_state();
    draw_image(&state.title.as_image(), Point { x: 0, y: 0 });
    display_centered_message(Some(Color::White), &["Lampy", "Press <E>"]);
}

pub fn render_died() {
    let state = get_state();
    state.draw();
    display_centered_message(None, &["You died!", "Press <E>"]);
}

pub fn render_gameover(won: bool) {
    let state = get_state();
    state.draw();
    if won {
        display_centered_message(None, &["You win!", "Press <E>"]);
    } else {
        display_centered_message(None, &["You lost!", "Press <E>"]);
    }
}

pub fn render_ui() {
    let state = get_state();
    state
        .players
        .iter()
        .find(|player| Some(player.peer) == state.me)
        .map(|player| {
            display_text_color(
                format!("Points: {}", player.points).as_str(),
                Point::new(0, 14),
                Palette::Black.into(),
            );
        });
}

pub fn render_credits() {
    clear_screen(Color::White);
    display_left_message(&CREDITS);
}

pub fn render_info() {
    clear_screen(Color::White);
    display_left_message(&INFO);
}
