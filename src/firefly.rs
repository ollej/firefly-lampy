use firefly_rust::{draw_line, draw_point, log_debug, math, Angle, LineStyle, Point};

use alloc::format;

use crate::{
    camera::Camera, constants::WORLD_HEIGHT, constants::WORLD_WIDTH, firefly_color::FireflyColor,
    palette::Palette, particles::ParticleSystem, player::Player, point_math::PointMath,
    state::get_state, utility::random_range, world::World,
};

pub struct Firefly {
    pub attracted_to: Option<Point>,
    pub color: FireflyColor,
    direction: Angle,
    particles: ParticleSystem,
    pub position: Point,
    remainder: f32,
    cached_pos: Option<Point>,
    cache_age: u8,
}

impl Firefly {
    pub const MAX_COUNT: i32 = 20;
    const ATTRACTION_DISTANCE: i32 = 40;
    const SPEED: f32 = 1.0;

    pub fn new_random(world: &World) -> Self {
        let color = FireflyColor::random();
        Firefly {
            attracted_to: None,
            color,
            direction: Self::random_direction(),
            particles: ParticleSystem::new(20),
            position: world.random_unblocked_point_in_rectangle(color.starting_rect()),
            remainder: 0.0,
            cached_pos: None,
            cache_age: 255,
        }
    }

    fn random_direction() -> Angle {
        Angle::from_degrees(random_range(0, 360) as f32)
    }

    pub fn update(&mut self, world: &World) {
        self.update_movement(world);
        self.spawn_particles();
    }

    pub fn draw(&self, camera: &Camera) {
        self.particles.render(camera);
        // Debug line from firefly to closest attraction_target
        let state = get_state();
        if state.debug {
            self.draw_debug_line_to_attraction_point(camera);
        }
        let transformed_position = camera.world_to_screen(self.position);
        draw_point(transformed_position, Palette::Black.into());
    }

    pub fn is_in_goal(&self, world: &World) -> bool {
        world.is_in_goal(self.position)
    }

    pub fn points(&self) -> i32 {
        self.color.points()
    }

    pub fn color(&self) -> Palette {
        self.color.color()
    }

    pub fn matches_player(&self, player: &Player) -> bool {
        player.color == Some(self.color())
    }

    fn update_movement(&mut self, world: &World) {
        if self.cache_age > 5 {
            self.cached_pos = self.find_closest_target();
            self.cache_age = 0;
        } else {
            self.cache_age += 1;
        }
        self.change_direction();

        // Skip movement if at attraction_target
        if Some(self.position) == self.attracted_to {
            //log_debug("at attraction_target");
            self.remainder = 0.0;
            return;
        }

        let (new_position, remainder) = self
            .position
            .point_from_distance_and_angle(Self::SPEED + self.remainder, self.direction);
        self.remainder = remainder;
        self.change_direction_on_wall_hit(new_position, world);

        if !world.is_blocked(new_position) {
            self.position = Point {
                x: new_position.x.clamp(0, WORLD_WIDTH - 1),
                y: new_position.y.clamp(0, WORLD_HEIGHT - 1),
            };
        }
    }

    fn change_direction_on_wall_hit(&mut self, new_position: Point, world: &World) {
        // Change direction when hitting walls
        if world.is_blocked(new_position) {
            let random_angle = Angle::from_degrees(30.0 + random_range(0, 120) as f32);
            if new_position.x > self.position.x {
                self.direction = Angle::QUARTER_CIRCLE + random_angle;
            } else if new_position.x < self.position.x {
                self.direction = (Angle::from_degrees(270.0) + random_angle).normalize();
            } else if new_position.y > self.position.y {
                self.direction = Angle::HALF_CIRCLE + random_angle;
            } else {
                self.direction = Angle::ZERO + random_angle;
            }
        }
    }

    fn change_direction(&mut self) {
        if let Some(attraction_target) = self.cached_pos {
            // Set direction towards closest attraction target within reach
            self.attracted_to = Some(attraction_target);
            self.direction = self.position.angle_to(&attraction_target);
        } else if self.attracted_to.is_some() {
            //log_debug("lost attraction_target");
            self.attracted_to = None;
            self.direction = Self::random_direction();
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
            .filter(|player| self.matches_player(player))
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

    fn draw_debug_line_to_attraction_point(&self, camera: &Camera) {
        if let Some(attraction_target) = self.attracted_to {
            let from = camera.world_to_screen(self.position);
            let to = camera.world_to_screen(attraction_target);
            draw_line(
                from,
                to,
                LineStyle {
                    color: Palette::Black.into(),
                    width: 1,
                },
            );
        }
    }

    fn spawn_particles(&mut self) {
        self.spawn_trail_particles();
        self.spawn_firefly_flash_particles();
        self.particles.update();
    }

    fn spawn_trail_particles(&mut self) {
        if random_range(0, 60) < 20 {
            self.particles.spawn(
                self.position.x,
                self.position.y,
                0,
                0,
                30,
                self.color().into(),
                1,
            );
        }
    }

    fn spawn_firefly_flash_particles(&mut self) {
        if random_range(0, 60) < 10 {
            self.particles.spawn_radial_burst(
                self.position.x,
                self.position.y,
                random_range(8, 14) as u8,
                random_range(1, 2) as i16,
                2,
                self.color().into(),
            );
        }
    }
}
