use crate::{palette::*, utility::random_range};

pub enum FireflyColor {
    SoftRed,
    BrightMagenta,
    BrightGreen,
    BrightBlue,
    None,
}

impl FireflyColor {
    pub const fn new(v: u8) -> Self {
        match v {
            0 => FireflyColor::SoftRed,
            1 => FireflyColor::BrightMagenta,
            2 => FireflyColor::BrightGreen,
            3 => FireflyColor::BrightBlue,
            _ => FireflyColor::None,
        }
    }

    pub fn random() -> Self {
        let idx = random_range(0, 3) as u8;
        FireflyColor::new(idx)
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
