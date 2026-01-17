use firefly_rust::{Angle, Color, Point, draw_point, log_debug};

use crate::{particles::*, point_math::*, utility::*};

pub struct Firefly {
    direction: Angle,
    position: Point,
    color: Color,
    particles: ParticleSystem,
}

impl Firefly {
    pub const MAX_COUNT: i32 = 20;
    const SPEED: f32 = 1.0;
    const COLORS: [Color; 4] = [
        Color::Purple,
        Color::LightGreen,
        Color::Yellow,
        Color::LightBlue,
    ];

    pub fn new() -> Self {
        Firefly {
            color: Color::Yellow,
            direction: Angle::ZERO,
            particles: ParticleSystem::new(20),
            position: Point::new(10, 10),
        }
    }

    pub fn random() -> Self {
        Firefly {
            color: Self::random_color(),
            direction: Angle::from_degrees(random_range(0, 360) as f32),
            particles: ParticleSystem::new(20),
            position: Point::new(random_range(0, 240) as i32, random_range(0, 160) as i32),
        }
    }

    pub fn update(&mut self) {
        self.update_movement();
        self.spawn_particles();
    }

    fn update_movement(&mut self) {
        let random_direction_change = Angle::from_degrees(random_range(0, 10) as f32 - 5.0);
        self.direction = self.direction + random_direction_change;
        let new_position = self
            .position
            .point_from_distance_and_angle(Self::SPEED, self.direction);
        self.position = Point {
            x: new_position.x.clamp(0, 240),
            y: new_position.y.clamp(0, 160),
        };
    }

    fn spawn_particles(&mut self) {
        // Spawn firefly flash
        let color = if random_range(1, 2) == 1 {
            Color::Yellow
        } else {
            Color::Orange
        };
        self.particles.spawn_radial_burst(
            self.position.x,
            self.position.y,
            random_range(10, 15) as u8,
            random_range(1, 2) as i16,
            2,
            color,
        );
        // Spawn trail
        if random_range(0, 60) < 20 {
            self.particles.spawn(
                self.position.x,
                self.position.y,
                20,
                20,
                60,
                Color::LightGray,
                1,
            );
        }
        self.particles.update();
    }

    pub fn draw(&self) {
        self.particles.render();
        draw_point(self.position, self.color);
    }

    fn random_color() -> Color {
        let idx = random_range(0, 3) as usize;
        *Self::COLORS.get(idx).unwrap_or(&Color::Purple)
    }
}
