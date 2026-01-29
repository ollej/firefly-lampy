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
    "Music + SFX:",
    " -OpenGameArt",
];

pub const INFO: [&str; 6] = [
    "Guide the fireflies to the score pad in the",
    " middle of the map to score points.",
    "Attract fireflies by shining a flashlight",
    " of the same color at them.",
    "Steer your circle with touchpad.",
    "Use buttons to shine flash light.",
];

pub const TILE_WIDTH: i32 = 16;
pub const TILE_HEIGHT: i32 = 16;
pub const SPRITES_W: i32 = 16;
pub const SPRITES_H: i32 = 16;
pub const TILES_H: i32 = 30;
pub const FONT_LARGE_HEIGHT: i32 = 16;
pub const FONT_LARGE_HALF_WIDTH: i32 = 8;
pub const FONT_SMALL_HEIGHT: i32 = 8;
pub const BADGE_WINS: Badge = Badge(1);
pub const PI: f32 = f32::consts::PI;
pub const WORLD_WIDTH: i32 = 480;
pub const WORLD_HEIGHT: i32 = 480;
pub const SCREEN_WIDTH: i32 = 240;
pub const SCREEN_HEIGHT: i32 = 160;
