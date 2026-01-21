use core::f32;

use firefly_rust::Badge;

pub const CREDITS: [&str; 9] = [
    "<Credits>",
    "",
    "Programming:",
    " -Olle Wreede",
    " -Catboots",
    "Graphics:",
    " -Catboots",
    "Music: ??",
    "SFX: ??",
];

pub const INFO: [&str; 8] = [
    "<Controls>",
    "Steer with",
    " touchpad.",
    "Use buttons to",
    " shine flash",
    " light.",
    "Guide fireflies",
    " to score pad",
];

pub const TILE_WIDTH: i32 = 16;
pub const TILE_HEIGHT: i32 = 16;
pub const SPRITES_W: i32 = 16;
pub const SPRITES_H: i32 = 16;
pub const TILES_H: i32 = 30;
pub const HALF_FONT_WIDTH: i32 = 8;
pub const FONT_BASE_LINE: i32 = 16;
pub const LINE_HEIGHT: i32 = 16;
pub const BADGE_WINS: Badge = Badge(1);
pub const BADGE_DEATHS: Badge = Badge(2);
pub const PI: f32 = f32::consts::PI;
pub const WORLD_WIDTH: i32 = 480;
pub const WORLD_HEIGHT: i32 = 480;
pub const SCREEN_WIDTH: i32 = 240;
pub const SCREEN_HEIGHT: i32 = 160;
