use alloc::format;
use firefly_rust::{
    draw_circle, draw_triangle, log_debug, read_buttons, read_pad, Angle, Buttons, Color, Peer,
    Point, Style,
};

use crate::{
    camera::*, constants::PI, constants::WORLD_HEIGHT, constants::WORLD_WIDTH, palette::*,
    point_math::*, world::*,
};

pub struct Player {
    pub attraction_target: Point,
    buttons: Buttons,
    pub color: Option<Palette>,
    direction: Angle,
    pub peer: Peer,
    pub points: i32,
    pub position: Point,
    remainder: f32,
    speed: f32,
    pub camera: Camera,
}

impl Player {
    const CONE_ANGLE: Angle = Angle::from_radians(PI / 20.0);
    const CONE_LENGTH: f32 = 25.0;
    const ATTRACTION_LENGTH: f32 = 20.0;
    const SPEED: f32 = 0.002;

    pub fn new(peer: Peer) -> Self {
        let direction = Angle::ZERO;
        let position = Point::new(120, 80);
        Self {
            attraction_target: Self::calculate_attraction_target(position, direction),
            buttons: Buttons::default(),
            color: None,
            direction,
            peer,
            points: 0,
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
            // Handle when azimuth is NaN
            if self.direction.to_radians().is_nan() {
                //log_debug("is_nan!");
                self.direction = Angle::ZERO;
                self.speed = 0.0;
            }
            /*
            log_debug(
                format!(
                    "direction: {} speed: {} remainder: {} position: {:?}",
                    self.direction.to_degrees(),
                    self.speed,
                    self.remainder,
                    self.position
                )
                .as_str(),
            );
            */
            if self.speed > 0.0 {
                let (new_position, remainder) = self.position.point_from_distance_and_angle(
                    self.speed * Self::SPEED + self.remainder,
                    self.direction,
                );
                if new_position.x <= 0
                    || new_position.x >= world.pixel_width
                    || new_position.y <= 0
                    || new_position.y >= world.pixel_height
                {
                    log_debug(format!("new position outside world: {:?}", new_position).as_str());
                }
                self.remainder = remainder;
                if !world.is_blocked(new_position) {
                    self.position = Point {
                        x: new_position.x.clamp(0, world.pixel_width - 1),
                        y: new_position.y.clamp(0, world.pixel_height - 1),
                    }
                };
                self.attraction_target =
                    Self::calculate_attraction_target(self.position, self.direction);
            }

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
            self.color = Some(Palette::SoftRed);
        }
        if just_pressed.e {
            self.color = Some(Palette::BrightMagenta);
        }
        if just_pressed.s {
            self.color = Some(Palette::BrightGreen);
        }
        if just_pressed.w {
            self.color = Some(Palette::BrightBlue);
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
            6,
            Style {
                fill_color: self.color.unwrap_or(Palette::Black).into(),
                stroke_color: Palette::Black.into(),
                stroke_width: 1,
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
                    fill_color: color.into(),
                    stroke_color: color.into(),
                    stroke_width: 0,
                },
            );
        }
    }
}
