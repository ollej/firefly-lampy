use alloc::vec;
use alloc::vec::Vec;

use crate::{camera::*, firefly::*, utility::*, world::*};

pub struct Fireflies {
    fireflies: Vec<Firefly>,
}

impl Fireflies {
    pub fn new() -> Self {
        Self { fireflies: vec![] }
    }

    pub fn update(&mut self, world: &World) -> Vec<Firefly> {
        if self.fireflies.len() < Firefly::MAX_COUNT as usize && random_range(0, 100) < 10 {
            self.fireflies.push(Firefly::new_random(world));
        }
        for firefly in self.fireflies.iter_mut() {
            firefly.update(world);
        }
        self.collect_fireflies(world)
    }

    pub fn draw(&self, camera: &Camera) {
        for firefly in self.fireflies.iter() {
            firefly.draw(camera);
        }
    }

    fn collect_fireflies(&mut self, world: &World) -> Vec<Firefly> {
        self.fireflies
            .extract_if(.., |firefly| Self::should_collect_firefly(firefly, world))
            .collect()
    }

    fn should_collect_firefly(firefly: &Firefly, world: &World) -> bool {
        firefly.attracted_to.is_some() && firefly.is_in_goal(world)
    }
}
