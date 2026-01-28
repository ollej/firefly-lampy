use alloc::format;
use firefly_rust::{clear_screen, draw_image, Color, Point};

use crate::{constants::*, drawing::*, palette::*, point_math::*, state::*};

pub fn render_title() {
    clear_screen(Palette::LightPurple.into());
    //let state = get_state();
    //draw_image(&state.title.as_image(), Point { x: 0, y: 0 });
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
            let text = format!("Points:{}", player.points);
            display_large_text_color(text.as_str(), Point::new(0, 14), Palette::Black.into());
            display_large_text_color(
                text.as_str(),
                Point::new(-1, 13),
                Palette::LightPurple.into(),
            );
        });
    display_large_text_color("1", Point::new(0, 160), Palette::SoftRed.into());
    display_large_text_color("2", Point::new(72, 160), Palette::BrightMagenta.into());
    display_large_text_color("3", Point::new(152, 160), Palette::BrightGreen.into());
    display_large_text_color("5", Point::new(224, 160), Palette::BrightBlue.into());
}

pub fn render_credits() {
    clear_screen(Palette::LightPurple.into());
    display_large_left_message(&CREDITS, Palette::Black.into());
}

pub fn render_info() {
    clear_screen(Palette::LightPurple.into());
    display_large_text_color(
        "<Controls>",
        Point {
            x: 8,
            y: FONT_LARGE_HEIGHT,
        },
        Palette::Black.into(),
    );
    display_small_left_message(
        &INFO,
        Point::new(12, FONT_LARGE_HEIGHT),
        Palette::Black.into(),
    );
    let pos = Point::new(12, 72);
    display_small_text_color(" (N) Red", pos, Palette::SoftRed.into());
    display_small_text_color(" (E) Purple", pos.addy(8), Palette::BrightMagenta.into());
    display_small_text_color(" (S) Green", pos.addy(16), Palette::BrightGreen.into());
    display_small_text_color(" (W) Blue", pos.addy(24), Palette::BrightBlue.into());

    let pos = Point::new(12, 104);
    display_large_text_color(
        "<Points>",
        pos.addy(FONT_LARGE_HEIGHT).addx(-4),
        Palette::Black.into(),
    );
    display_small_text_color("+ Red firefly: 1", pos.addy(24), Palette::SoftRed.into());
    display_small_text_color(
        "+ Purple firefly: 2",
        pos.addy(32),
        Palette::BrightMagenta.into(),
    );
    display_small_text_color(
        "+ Green firefly: 3",
        pos.addy(40),
        Palette::BrightGreen.into(),
    );
    display_small_text_color(
        "+ Blue firefly: 5",
        pos.addy(48),
        Palette::BrightBlue.into(),
    );
}
