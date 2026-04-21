use alloc::format;
use firefly_rust::{Color, Point, clear_screen};

use crate::{drawing::*, palette::*, state::*};

pub fn render_title() {
    clear_screen(Palette::LightPurple.into());
    //let state = get_state();
    //draw_image(&state.title.as_image(), Point { x: 0, y: 0 });
    display_centered_message(Some(Color::White), &["Lampy", "Press <E>"]);
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
        .find(|player| player.peer == state.me)
        .iter()
        .for_each(|player| {
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
