use firefly_rust::{draw_point, get_random, log_debug, Color, Point};

use crate::{particles::*, utility::*};

pub struct Firefly {
    position: Point,
    color: Color,
    particles: ParticleSystem,
}

impl Firefly {
    pub fn new() -> Self {
        Firefly {
            position: Point::new(10, 10),
            color: Color::Yellow,
            particles: ParticleSystem::new(20),
        }
    }

    pub fn update(&mut self) {
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
}
