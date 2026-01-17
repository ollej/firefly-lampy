use firefly_rust::{draw_point, log_debug, math, Angle, Color, Point};

use crate::{particles::*, point_math::*, state::*, utility::*};

pub struct Firefly {
    direction: Angle,
    position: Point,
    color: Color,
    particles: ParticleSystem,
}

impl Firefly {
    pub const MAX_COUNT: i32 = 20;
    const ATTRACTION_DISTANCE: i32 = 10;
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
        //self.spawn_particles();
    }

    fn update_movement(&mut self) {
        let direction_change = self.direction_change();
        self.direction = self.direction + direction_change;
        let new_position = self
            .position
            .point_from_distance_and_angle(Self::SPEED, self.direction);
        self.position = Point {
            x: new_position.x.clamp(0, 240),
            y: new_position.y.clamp(0, 160),
        };
    }

    fn direction_change(&self) -> Angle {
        if let Some(attraction_target) = self.find_closest_target() {
            self.position.angle_to(&attraction_target)
        } else {
            Angle::from_degrees(random_range(0, 10) as f32 - 5.0)
        }
    }

    fn find_closest_target(&self) -> Option<Point> {
        let state = get_state();
        state
            .players
            .iter()
            .filter(|player| player.color.is_some())
            .filter(|player| self.color == player.color.unwrap())
            .map(|player| {
                (
                    player.attraction_target,
                    math::floor(self.position.distance(&player.attraction_target)) as i32,
                )
            })
            .filter(|(_attraction_target, distance)| distance < &Self::ATTRACTION_DISTANCE)
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(attraction_target, _distance)| attraction_target)
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
