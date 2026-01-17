use crate::utility::random_range;
use alloc::vec::Vec;
use firefly_rust::{Color, Point, draw_point, get_random, log_debug};

const SCREEN_WIDTH: i32 = 240;
const SCREEN_HEIGHT: i32 = 160;

const GRAVITY: i16 = 0;

const FIXED_POINT_SHIFT: i16 = 4;

const SIN_TABLE: [i16; 16] = [
    0, 6, 11, 15, 16, 15, 11, 6, 0, -6, -11, -15, -16, -15, -11, -6,
];
const COS_TABLE: [i16; 16] = [
    16, 15, 11, 6, 0, -6, -11, -15, -16, -15, -11, -6, 0, 6, 11, 15,
];

#[derive(Clone, Copy)]
pub struct Particle {
    pub x: i32,
    pub y: i32,
    pub vx: i16,
    pub vy: i16,
    pub lifetime: u8,
    pub max_lifetime: u8,
    pub color: Color,
    pub size: u8,
}

impl Particle {
    fn calculate_lifetime(&self) -> u8 {
        if self.max_lifetime == 0 {
            return 0;
        }

        ((self.lifetime as u16 * 255) / self.max_lifetime as u16) as u8
    }
}

#[derive(Clone)]
pub struct ParticleSystem {
    particles: Vec<Particle>,
    max_particles: usize,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
        }
    }

    pub fn update(&mut self) {
        let mut i = 0;
        while i < self.particles.len() {
            let p = &mut self.particles[i];

            p.x += (p.vx) as i32;
            p.y += (p.vy) as i32;

            //p.vy = p.vy.saturating_add(GRAVITY);

            p.lifetime = p.lifetime.saturating_sub(1);

            let out_of_bounds =
                p.x < -5 || p.x > SCREEN_WIDTH + 5 || p.y < -5 || p.y > SCREEN_HEIGHT + 5;

            if p.lifetime <= 0 || out_of_bounds {
                self.particles.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn render(&self) {
        for p in &self.particles {
            draw_point(Point { x: p.x, y: p.y }, p.color);
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
        if self.particles.len() >= self.max_particles {
            return; // TODO: Implent recycling
        }

        self.particles.push(Particle {
            x,
            y,
            vx,
            vy,
            lifetime,
            max_lifetime: lifetime,
            color,
            size,
        });
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
        for i in 0..count {
            if self.particles.len() >= self.max_particles {
                break;
            }

            let rand_num = random_range(0, 15);
            let dir = (rand_num) as usize;
            let vx = (speed * COS_TABLE[dir]);
            let vy = (speed * SIN_TABLE[dir]);
            self.spawn(x, y, vx, vy, lifetime, color, 1);
        }
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }
}
