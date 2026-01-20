use alloc::string::String;

use firefly_rust::{draw_text, Color, Point};

use crate::{camera::Camera, state::get_state};

pub struct Text {
    age: i32,
    color: Color,
    content: String,
    position: Point,
}

impl Text {
    const TEXT_SPEED: f32 = 0.1;
    const MAX_AGE: i32 = 30;

    pub fn new(content: String, color: Color, position: Point) -> Self {
        Self {
            age: 0,
            content,
            color,
            position,
        }
    }

    pub fn update(&mut self) {
        self.age += 1;
        self.position.y -= 1;
    }

    pub fn draw(&self, camera: &Camera) {
        let state = get_state();
        let font = state.font.as_font();
        let position = camera.world_to_screen(self.position);
        draw_text(self.content.as_str(), &font, position, self.color);
    }

    pub fn remove(&self) -> bool {
        self.age > Self::MAX_AGE
    }
}
