use alloc::format;
use firefly_rust::{
    draw_circle, draw_triangle, log_debug, read_buttons, read_pad, Angle, Buttons, Peer, Point,
    Style,
};

use crate::{
    camera::*, constants::PI, constants::WORLD_HEIGHT, constants::WORLD_WIDTH, palette::*,
    point_math::*, utility::movement_to_step, world::*,
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
}

impl Player {
    const CONE_ANGLE: Angle = Angle::from_radians(PI / 20.0);
    const CONE_LENGTH: f32 = 25.0;
    const ATTRACTION_LENGTH: f32 = 20.0;
    const SPEED: f32 = 0.002;
    const FLASHLIGHT_SPEED: f32 = 0.4;

    pub fn new(peer: Peer, world: &World) -> Self {
        let direction = Angle::ZERO;
        let position = world.random_unblocked_point();
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
        }
    }

    pub fn update(&mut self, world: &World) {
        self.update_position(world);
        self.update_light_cone();
    }

    pub fn draw(&self, camera: &Camera) {
        self.draw_light_cone(camera);
        self.draw_lamp(camera);
    }

    pub fn reset(&mut self, world: &World) {
        self.points = 0;
        self.position = world.random_unblocked_point();
    }

    fn calculate_attraction_target(position: Point, direction: Angle) -> Point {
        let (new_position, _remainder) =
            position.point_from_distance_and_angle(Self::ATTRACTION_LENGTH, direction);
        new_position
    }

    fn update_position(&mut self, world: &World) {
        if let Some(pad) = read_pad(self.peer) {
            let speed = pad.radius();
            if speed <= 0.01 {
                self.direction = Angle::ZERO;
                self.speed = 0.0;
            } else {
                self.direction = -pad.azimuth();
                self.speed = speed;
            }
            if self.speed > 100.0 {
                // Slow down player when using flash light
                let flashlight_speed = self.color.map(|_| Self::FLASHLIGHT_SPEED).unwrap_or(1.0);
                let distance = self.speed * Self::SPEED * flashlight_speed + self.remainder;

                let (new_position, remainder) = self
                    .position
                    .point_from_distance_and_angle(distance, self.direction);
                self.debug_teleport(new_position);
                self.remainder = remainder;
                let old_position = self.position;
                self.move_horizontally(new_position, world);
                self.move_vertically(new_position, world);
                if self.position != old_position {
                    self.attraction_target =
                        Self::calculate_attraction_target(self.position, self.direction);
                };
            } else {
                self.remainder = 0.0;
            }
        }
    }

    fn move_horizontally(&mut self, target_position: Point, world: &World) {
        let amount = target_position.x - self.position.x;
        let step = movement_to_step(amount);
        for _ in 0..amount.abs() as i32 {
            let test_pos = self.position.addx(step);
            if !world.is_blocked(test_pos) {
                self.position.x = test_pos.x;
            }
        }
    }

    fn move_vertically(&mut self, target_position: Point, world: &World) {
        let amount = target_position.y - self.position.y;
        let step = movement_to_step(amount);
        for _ in 0..amount.abs() as i32 {
            let test_pos = self.position.addy(step);
            if !world.is_blocked(test_pos) {
                self.position.y = test_pos.y;
            }
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

    fn draw_lamp(&self, camera: &Camera) {
        let transformed_position: Point = camera.world_to_screen(self.position);

        draw_circle(
            Point {
                x: transformed_position.x - 2,
                y: transformed_position.y - 2,
            },
            6,
            Style {
                fill_color: self.color.unwrap_or(Palette::Yellow).into(),
                stroke_color: Palette::Black.into(),
                stroke_width: 1,
            },
        );
    }

    fn draw_light_cone(&self, camera: &Camera) {
        if let Some(color) = self.color {
            let a = camera.world_to_screen(self.position);
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

    fn debug_teleport(&self, position: Point) {
        if position.x <= 0
            || position.x >= WORLD_WIDTH
            || position.y <= 0
            || position.y >= WORLD_HEIGHT
        {
            log_debug(
                format!(
                    "new position outside world: {:?} speed: {} remainder: {} distance: {} direction: {:?}",
                    position, self.speed, self.remainder, self.speed * Self::SPEED + self.remainder, self.direction
                )
                .as_str(),
            );
        }
    }
}
