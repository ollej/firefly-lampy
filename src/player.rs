use firefly_rust::{draw_triangle, log_debug, Color, Peer, Point, Style};

#[derive(Clone, Copy)]
pub struct Player {
    position: Point,
    direction: f32,
    speed: f32,
    color: Color,
    peer: Option<Peer>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Point::new(120, 80),
            direction: 0.0,
            speed: 0.0,
            color: Color::White,
            peer: None,
        }
    }
}

impl Player {
    pub fn new(peer: Peer) -> Self {
        Self {
            peer: Some(peer),
            ..Self::default()
        }
    }

    pub fn draw(&self) {
        draw_triangle(
            Point { x: 60, y: 10 },
            Point { x: 40, y: 40 },
            Point { x: 80, y: 40 },
            Style {
                fill_color: Color::LightGray,
                stroke_color: Color::DarkBlue,
                stroke_width: 1,
            },
        );
    }
}
