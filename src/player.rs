use firefly_rust::{draw_triangle, log_debug, read_pad, Angle, Color, Peer, Point, Style};

use crate::{constants::PI, point_math::*};

pub struct Player {
    position: Point,
    direction: Angle,
    speed: f32,
    color: Color,
    peer: Peer,
}

impl Player {
    const CONE_ANGLE: Angle = Angle::from_radians(PI / 20.0);
    const CONE_LENGTH: f32 = 25.0;

    pub fn new(peer: Peer) -> Self {
        Self {
            color: Color::White,
            direction: Angle::ZERO,
            peer,
            position: Point::new(120, 80),
            speed: 0.0,
        }
    }

    pub fn update(&mut self) {
        if let Some(pad) = read_pad(self.peer) {
            self.direction = pad.azimuth();
        }
    }

    pub fn draw(&self) {
        let a = self.position;
        let b =
            a.point_from_distance_and_angle(Self::CONE_LENGTH, self.direction - Self::CONE_ANGLE);
        let c =
            a.point_from_distance_and_angle(Self::CONE_LENGTH, self.direction + Self::CONE_ANGLE);
        draw_triangle(
            a,
            b,
            c,
            Style {
                fill_color: Color::Yellow,
                stroke_color: Color::Yellow,
                stroke_width: 0,
            },
        );
    }
}
