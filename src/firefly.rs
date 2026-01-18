use firefly_rust::{draw_line, draw_point, math, Angle, Color, LineStyle, Point};

use crate::{camera::*, particles::*, point_math::*, state::*, utility::*, world::*};

pub struct Firefly {
    color: Color,
    direction: Angle,
    particles: ParticleSystem,
    position: Point,
    remainder: f32,
}

impl Firefly {
    pub const MAX_COUNT: i32 = 100;
    const ATTRACTION_DISTANCE: i32 = 20;
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
            remainder: 0.0,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Firefly {
            color: Self::random_color(),
            direction: Angle::from_degrees(random_range(0, 360) as f32),
            particles: ParticleSystem::new(20),
            position: Point::new(
                random_range(0, width as u32) as i32,
                random_range(0, height as u32) as i32,
            ),
            remainder: 0.0,
        }
    }

    pub fn update(&mut self, world: &World) {
        self.update_movement(world);
        //self.spawn_particles();
    }

    fn update_movement(&mut self, world: &World) {
        self.change_direction();
        let (new_position, remainder) = self
            .position
            .point_from_distance_and_angle(Self::SPEED + self.remainder, self.direction);
        self.remainder = remainder;
        self.change_direction_on_wall_hit(new_position, world);

        if !world.is_blocked(new_position) {
            self.position = Point {
                x: new_position.x.clamp(0, world.pixel_width - 1),
                y: new_position.y.clamp(0, world.pixel_height - 1),
            };
        }
    }

    fn change_direction_on_wall_hit(&mut self, new_position: Point, world: &World) {
        // Change direction when hitting walls
        let posx = Point {
            x: new_position.x,
            y: self.position.y,
        };
        if world.is_blocked(posx) || new_position.x < 0 || new_position.x > (world.pixel_width - 1)
        {
            let new_direction = Angle::HALF_CIRCLE - self.direction;
            self.direction = new_direction.normalize();
        }
        let posy = Point {
            x: self.position.x,
            y: new_position.y,
        };
        if world.is_blocked(posy) || new_position.y < 0 || new_position.y > (world.pixel_height - 1)
        {
            let new_direction = Angle::FULL_CIRCLE - self.direction;
            self.direction = new_direction.normalize();
        }
    }

    fn change_direction(&mut self) {
        if let Some(attraction_target) = self.find_closest_target() {
            // Set direction towards closest attraction target within reach
            self.direction = self.position.angle_to(&attraction_target);
        } else {
            // Change direction randomly +/- degrees
            let direction_change = Angle::from_degrees(random_range(0, 10) as f32 - 5.0);
            let new_direction = self.direction + direction_change;
            self.direction = new_direction.normalize();
        }
    }

    fn find_closest_target(&self) -> Option<Point> {
        let state = get_state();
        state
            .players
            .iter()
            .filter(|player| Some(self.color) == player.color)
            .map(|player| {
                (
                    player.attraction_target,
                    self.distance_to(player.attraction_target),
                )
            })
            .filter(|(_attraction_target, distance)| self.within_attraction_distance(distance))
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(attraction_target, _distance)| attraction_target)
    }

    fn within_attraction_distance(&self, distance: &i32) -> bool {
        distance < &Self::ATTRACTION_DISTANCE
    }

    fn distance_to(&self, point: Point) -> i32 {
        math::floor(self.position.distance(&point)) as i32
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

    pub fn draw(&self, camera: &Camera) {
        self.particles.render(camera);
        // Debug line from firefly to closest attraction_target
        self.draw_debug_line_to_attraction_point(camera);
        let transformed_position = camera.world_to_screen(self.position);
        draw_point(transformed_position, self.color);
    }

    fn draw_debug_line_to_attraction_point(&self, camera: &Camera) {
        if let Some(attraction_target) = self.find_closest_target() {
            let from = camera.world_to_screen(self.position);
            let to = camera.world_to_screen(attraction_target);
            draw_line(
                from,
                to,
                LineStyle {
                    color: Color::Black,
                    width: 1,
                },
            );
        }
    }

    fn random_color() -> Color {
        let idx = random_range(0, 3) as usize;
        *Self::COLORS.get(idx).unwrap_or(&Color::Purple)
    }
}
