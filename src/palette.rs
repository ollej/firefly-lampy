use firefly_rust::Color;

#[derive(PartialEq, Copy, Clone)]
pub enum Palette {
    None,
    DarkGreen,     // 1 = 0x213b25
    Green,         // 2 = 0x3a604a
    LightGreen,    // 3 = 0x4f7754
    LightYellow,   // 4 = 0xa19f7c
    Yellow,        // 5 = 0x77744f
    LightBrown,    // 6 = 0x775c4f
    Brown,         // 7 = 0x603b3a
    DarkPurple,    // 8 = 0x3b2137
    Black,         // 9 = 0x170e19
    Purple,        // 10 = 0x2f213b
    LightPurple,   // 11 = 0x433a60
    DarkBlue,      // 12 = 0x4f5277
    SoftRed,       // 13 = 0xde5d49
    BrightMagenta, // 14 = 0xe533e8
    BrightGreen,   // 15 = 0x70eb44
    BrightBlue,    // 16 = 0x43b1e8
}

impl From<Palette> for Color {
    fn from(value: Palette) -> Self {
        Self::new(value as u8)
    }
}
