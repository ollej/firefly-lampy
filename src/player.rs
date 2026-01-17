use firefly_rust::{
    Angle, Buttons, Color, Peer, Point, Style, draw_circle, draw_triangle, log_debug, read_buttons,
    read_pad,
};

use crate::{constants::PI, point_math::*};

pub struct Player {
    buttons: Buttons,
    color: Option<Color>,
    direction: Angle,
    pub peer: Peer,
    position: Point,
    speed: f32,
}

impl Player {
    const CONE_ANGLE: Angle = Angle::from_radians(PI / 20.0);
    const CONE_LENGTH: f32 = 25.0;
    const SPEED: f32 = 0.001;

    pub fn new(peer: Peer) -> Self {
        Self {
            buttons: Buttons::default(),
            color: None,
            direction: Angle::ZERO,
            peer,
            position: Point::new(120, 80),
            speed: 0.0,
        }
    }

    pub fn update(&mut self) {
        // Read touchpad
        if let Some(pad) = read_pad(self.peer) {
            self.direction = -pad.azimuth();
            self.speed = pad.radius();
            self.position = self
                .position
                .point_from_distance_and_angle(self.speed * Self::SPEED, self.direction)
        }

        // Read buttons
        let buttons = read_buttons(self.peer);
        let just_pressed = buttons.just_pressed(&self.buttons);
        //let just_released = buttons.just_released(&self.buttons);
        self.buttons = buttons;
        if just_pressed.n {
            self.color = Some(Color::Purple);
        }
        if just_pressed.e {
            self.color = Some(Color::LightGreen);
        }
        if just_pressed.s {
            self.color = Some(Color::Yellow);
        }
        if just_pressed.w {
            self.color = Some(Color::LightBlue);
        }
        if !self.buttons.any() {
            self.color = None;
        }
    }

    pub fn draw(&self) {
        self.draw_light_cone();
        self.draw_lamp();
    }

    fn draw_lamp(&self) {
        draw_circle(
            Point {
                x: self.position.x - 2,
                y: self.position.y - 2,
            },
            5,
            Style {
                fill_color: Color::Black,
                stroke_color: Color::Black,
                stroke_width: 0,
            },
        );
    }

    fn draw_light_cone(&self) {
        if let Some(color) = self.color {
            let a = self.position;
            let b = a.point_from_distance_and_angle(
                Self::CONE_LENGTH,
                self.direction - Self::CONE_ANGLE,
            );
            let c = a.point_from_distance_and_angle(
                Self::CONE_LENGTH,
                self.direction + Self::CONE_ANGLE,
            );
            draw_triangle(
                a,
                b,
                c,
                Style {
                    fill_color: color,
                    stroke_color: color,
                    stroke_width: 0,
                },
            );
        }
    }
}
