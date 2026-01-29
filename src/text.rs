use alloc::string::String;

use firefly_rust::{Color, Point, draw_text, math};

use crate::{camera::Camera, palette::Palette, state::get_state};

pub struct Text {
    age: i32,
    color: Color,
    content: String,
    position: Point,
    remainder: f32,
}

impl Text {
    const TEXT_SPEED: f32 = 0.6;
    const MAX_AGE: i32 = 30;

    pub fn new(content: String, color: Color, position: Point) -> Self {
        Self {
            age: 0,
            content,
            color,
            position,
            remainder: 0.0,
        }
    }

    pub fn update(&mut self) {
        self.age += 1;
        self.remainder += Self::TEXT_SPEED;
        let amount = math::floor(self.remainder + 0.5);
        self.remainder -= amount;
        self.position.y -= amount as i32;
    }

    pub fn draw(&self, camera: &Camera) {
        let state = get_state();
        let font = state.font_small.as_font();
        let position = camera.world_to_screen(self.position);
        let shadow_position = position + Point::new(1, 1);
        draw_text(
            self.content.as_str(),
            &font,
            shadow_position,
            Palette::Black.into(),
        );
        draw_text(self.content.as_str(), &font, position, self.color);
    }

    pub fn remove(&self) -> bool {
        self.age > Self::MAX_AGE
    }
}
