use firefly_rust::Badge;

pub const CREDITS: [&str; 7] = [
    "Credits:",
    "Programming: Olle Wreede & Catboots",
    "Graphics: Catboots",
    "Music: ??",
    "SFX: ??",
    "",
    "Press (E) to go back to game",
];

pub const INFO: [&str; 5] = [
    "Controls:",
    "Steer with touchpad",
    "Use buttons to flash light",
    "",
    "Press (E) to go back to game",
];

pub const TILE_WIDTH: i32 = 8;
pub const TILE_HEIGHT: i32 = 8;
pub const SPRITES_W: i32 = 16;
pub const SPRITES_H: i32 = 16;
pub const TILES_H: i32 = 30;
pub const HALF_FONT_WIDTH: i32 = 2;
pub const FONT_BASE_LINE: i32 = 4;
pub const LINE_HEIGHT: i32 = 8;
pub const BADGE_WINS: Badge = Badge(1);
pub const BADGE_DEATHS: Badge = Badge(2);
