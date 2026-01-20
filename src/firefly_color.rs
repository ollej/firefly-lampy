use crate::{palette::*, utility::random_range};

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
}
