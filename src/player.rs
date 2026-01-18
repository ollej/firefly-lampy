use alloc::format;
use firefly_rust::{
    draw_circle, draw_triangle, log_debug, read_buttons, read_pad, Angle, Buttons, Color, Peer,
    Point, Style,
};

use crate::{
    camera::*, constants::PI, constants::WORLD_HEIGHT, constants::WORLD_WIDTH, point_math::*,
    world::*,
};

pub struct Player {
    pub attraction_target: Point,
    buttons: Buttons,
    pub color: Option<Color>,
    direction: Angle,
    pub peer: Peer,
    position: Point,
    remainder: f32,
    speed: f32,
    pub camera: Camera,
}

impl Player {
    const CONE_ANGLE: Angle = Angle::from_radians(PI / 20.0);
    const CONE_LENGTH: f32 = 25.0;
    const ATTRACTION_LENGTH: f32 = 20.0;
    const SPEED: f32 = 0.001;

    pub fn new(peer: Peer) -> Self {
        let direction = Angle::ZERO;
        let position = Point::new(120, 80);
        Self {
            attraction_target: Self::calculate_attraction_target(position, direction),
            buttons: Buttons::default(),
            color: None,
            direction,
            peer,
            position,
            remainder: 0.0,
            speed: 0.0,
            camera: Camera::new(WORLD_WIDTH, WORLD_HEIGHT),
        }
    }

    fn calculate_attraction_target(position: Point, direction: Angle) -> Point {
        let (new_position, _remainder) =
            position.point_from_distance_and_angle(Self::ATTRACTION_LENGTH, direction);
        new_position
    }

    pub fn update(&mut self, world: &World) {
        self.update_position(world);
        self.update_light_cone();
    }

    fn update_position(&mut self, world: &World) {
        // Read touchpad
        if let Some(pad) = read_pad(self.peer) {
            self.direction = -pad.azimuth();
            self.speed = pad.radius();
            if self.direction.to_radians().is_nan() {
                log_debug("is_nan!");
                self.direction = Angle::ZERO;
                self.speed = 0.0;
            }
            log_debug(
                format!(
                    "direction: {} speed: {} position: {:?}",
                    self.direction.to_degrees(),
                    self.speed,
                    self.position
                )
                .as_str(),
            );
            let (new_position, remainder) = self.position.point_from_distance_and_angle(
                self.speed * Self::SPEED + self.remainder,
                self.direction,
            );
            self.remainder = remainder;
            if !world.is_blocked(new_position) {
                self.position = Point {
                    x: new_position.x.clamp(0, world.pixel_width),
                    y: new_position.y.clamp(0, world.pixel_height),
                }
            };
            self.attraction_target =
                Self::calculate_attraction_target(self.position, self.direction);

            //self.camera.set_camera_position(self.position);
            self.camera.follow_player(self.position, 0.2);
        }
    }

    fn update_light_cone(&mut self) {
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
        let transformed_position: Point = self.camera.world_to_screen(self.position);

        draw_circle(
            Point {
                x: transformed_position.x - 2,
                y: transformed_position.y - 2,
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
            let a = self.camera.world_to_screen(self.position);
            let (b, _) = a.point_from_distance_and_angle(
                Self::CONE_LENGTH,
                self.direction - Self::CONE_ANGLE,
            );
            let (c, _) = a.point_from_distance_and_angle(
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
