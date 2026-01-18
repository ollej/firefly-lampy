use firefly_rust::{get_random, set_color, Color, RGB};

pub fn random_range(min: u32, max: u32) -> u32 {
    if min >= max {
        return min;
    }
    let range = max - min + 1;
    min + (get_random() % range)
}

pub fn set_colors() {
    // https://lospec.com/palette-list/steam-lords
    set_color(Color::from(1), RGB::new(0x21, 0x3b, 0x25));
    set_color(Color::from(2), RGB::new(0x3a, 0x60, 0x4a));
    set_color(Color::from(3), RGB::new(0x4f, 0x77, 0x54));
    set_color(Color::from(4), RGB::new(0xa1, 0x9f, 0x7c));
    set_color(Color::from(5), RGB::new(0x77, 0x74, 0x4f));
    set_color(Color::from(6), RGB::new(0x77, 0x5c, 0x4f));
    set_color(Color::from(7), RGB::new(0x60, 0x3b, 0x3a));
    set_color(Color::from(8), RGB::new(0x3b, 0x21, 0x37));
    set_color(Color::from(9), RGB::new(0x17, 0x0e, 0x19));
    set_color(Color::from(10), RGB::new(0x2f, 0x21, 0x3b));
    set_color(Color::from(11), RGB::new(0x43, 0x3a, 0x60));
    set_color(Color::from(12), RGB::new(0x4f, 0x52, 0x77));
    set_color(Color::from(13), RGB::new(0x65, 0x73, 0x8c));
    set_color(Color::from(14), RGB::new(0x7c, 0x94, 0xa1));
    set_color(Color::from(15), RGB::new(0xa0, 0xb9, 0xba));
    set_color(Color::from(16), RGB::new(0xc0, 0xd1, 0xcc));
}