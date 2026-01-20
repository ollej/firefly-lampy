use firefly_rust::Point;

use crate::{
    constants::WORLD_HEIGHT, constants::WORLD_WIDTH, palette::Palette, rectangle::Rectangle,
    utility::random_range,
};

#[derive(Clone, Copy, Debug)]
pub enum FireflyColor {
    SoftRed,
    BrightMagenta,
    BrightGreen,
    BrightBlue,
    None,
}

impl FireflyColor {
    pub fn random() -> Self {
        match random_range(0, 36) {
            0..20 => FireflyColor::SoftRed,
            20..30 => FireflyColor::BrightMagenta,
            30..35 => FireflyColor::BrightGreen,
            35..=36 => FireflyColor::BrightBlue,
            _ => FireflyColor::None,
        }
    }

    pub fn color(&self) -> Palette {
        match self {
            FireflyColor::SoftRed => Palette::SoftRed,
            FireflyColor::BrightMagenta => Palette::BrightMagenta,
            FireflyColor::BrightGreen => Palette::BrightGreen,
            FireflyColor::BrightBlue => Palette::BrightBlue,
            FireflyColor::None => Palette::Black,
        }
    }

    pub fn points(&self) -> i32 {
        match self {
            FireflyColor::SoftRed => 1,
            FireflyColor::BrightMagenta => 2,
            FireflyColor::BrightGreen => 3,
            FireflyColor::BrightBlue => 5,
            FireflyColor::None => 0,
        }
    }

    pub fn starting_rect(&self) -> Rectangle {
        match self {
            FireflyColor::SoftRed => Rectangle::new(Point::new(16, 16), 192, 192),
            FireflyColor::BrightMagenta => Rectangle::new(Point::new(16, 256), 192, 192),
            FireflyColor::BrightGreen => Rectangle::new(Point::new(256, 16), 192, 192),
            FireflyColor::BrightBlue => Rectangle::new(Point::new(256, 256), 192, 192),
            FireflyColor::None => Rectangle::new(Point::new(0, 0), WORLD_WIDTH, WORLD_HEIGHT),
        }
    }
}
