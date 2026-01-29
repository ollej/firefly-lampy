use crate::camera::*;
use crate::constants::{WORLD_HEIGHT, WORLD_WIDTH};
use crate::utility::random_range;
use alloc::vec;
use alloc::vec::Vec;
use firefly_rust::{draw_point, Color, Point};

const FIXED_POINT_SHIFT: i16 = 4;

const SIN_TABLE: [i16; 16] = [
    0, 6, 11, 15, 16, 15, 11, 6, 0, -6, -11, -15, -16, -15, -11, -6,
];
const COS_TABLE: [i16; 16] = [
    16, 15, 11, 6, 0, -6, -11, -15, -16, -15, -11, -6, 0, 6, 11, 15,
];

#[derive(Default, Clone, Copy)]
pub struct Particle {
    pub x: i32,
    pub y: i32,
    pub vx: i16,
    pub vy: i16,
    pub lifetime: u8,
    pub max_lifetime: u8,
    pub color: Color,
    pub size: u8,
    pub active: bool,
}

impl Particle {
    fn calculate_lifetime(&self) -> u8 {
        if self.max_lifetime == 0 {
            return 0;
        }

        ((self.lifetime as u16 * 255) / self.max_lifetime as u16) as u8
    }

    fn deactivate_particle(&mut self) {
        self.active = false;
        self.lifetime = 0;
    }
}

#[derive(Clone)]
pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: vec![Particle::default(); max_particles],
        }
    }

    pub fn update(&mut self) {
        for particle in self.particles.iter_mut() {
            if !particle.active {
                continue;
            }

            particle.x += (particle.vx >> FIXED_POINT_SHIFT) as i32;
            particle.y += (particle.vy >> FIXED_POINT_SHIFT) as i32;

            particle.lifetime = particle.lifetime.saturating_sub(1);

            let out_of_bounds = particle.x < -5
                || particle.x > WORLD_WIDTH + 5
                || particle.y < -5
                || particle.y > WORLD_HEIGHT + 5;

            if particle.lifetime == 0 || out_of_bounds {
                particle.deactivate_particle();
            }
        }
    }

    pub fn render(&self, camera: &Camera) {
        for particle in self.particles.iter() {
            if !particle.active {
                continue;
            }

            let position = Point {
                x: particle.x,
                y: particle.y,
            };
            let transformed_position = camera.world_to_screen(position);
            draw_point(transformed_position, particle.color);
        }
    }

    pub fn spawn(
        &mut self,
        x: i32,
        y: i32,
        vx: i16,
        vy: i16,
        lifetime: u8,
        color: Color,
        size: u8,
    ) {
        if let Some(particle) = self.particles.iter_mut().find(|p| !p.active) {
            particle.x = x;
            particle.y = y;
            particle.vx = vx;
            particle.vy = vy;
            particle.lifetime = lifetime;
            particle.max_lifetime = lifetime;
            particle.color = color;
            particle.size = size;
            particle.active = true;
        }
    }

    pub fn spawn_radial_burst(
        &mut self,
        x: i32,
        y: i32,
        count: u8,
        speed: i16,
        lifetime: u8,
        color: Color,
    ) {
        let mut spawned_particles = 0;

        for particle in self.particles.iter_mut() {
            if spawned_particles >= count {
                break;
            }

            if particle.active {
                continue;
            }

            let dir = (random_range(0, 15)) as usize;
            let vx = speed * COS_TABLE[dir];
            let vy = speed * SIN_TABLE[dir];

            particle.x = x;
            particle.y = y;
            particle.vx = vx;
            particle.vy = vy;
            particle.lifetime = lifetime;
            particle.max_lifetime = lifetime;
            particle.color = color;
            particle.size = 1;
            particle.active = true;

            spawned_particles += 1;
        }
    }

    pub fn clear(&mut self) {
        for particle in self.particles.iter_mut() {
            particle.deactivate_particle();
        }
    }

    pub fn count(&self) -> usize {
        self.particles.iter().filter(|p| p.active).count()
    }
}
