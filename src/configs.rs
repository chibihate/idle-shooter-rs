use bevy::prelude::Vec2;

// Window
pub const WINDOW_WIDTH: f32 = 720.0;
pub const WINDOW_HEIGHT: f32 = 480.0;

// Sprites
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const TILE_WIDTH: u32 = 16;
pub const TILE_HEIGHT: u32 = 16;
pub const SPRITE_SHEET_WIDTH: u32 = 8;
pub const SPRITE_SHEET_HEIGHT: u32 = 8;

// Player
pub const MINIMUM_DISTANCE_CURSOR_PLAYER: f32 = 10.0;

// Colors
pub const BACKGROUND_COLOR: (u8, u8, u8) = (197, 204, 184);

// Weapon
pub const GUN_OFFSET: [Vec2; 6] = [
    Vec2::new(35.0, -15.0),
    Vec2::new(-45.0, -15.0),
    Vec2::new(35.0, 5.0),
    Vec2::new(-45.0, 5.0),
    Vec2::new(35.0, 25.0),
    Vec2::new(-45.0, 25.0),
];
// pub const WEAPON_POSITION_DYN: [Vec2; 6] = [
//     Vec2::new(15.0, -15.0),
//     Vec2::new(-25.0, -15.0),
//     Vec2::new(15.0, 5.0),
//     Vec2::new(-25.0, 5.0),
//     Vec2::new(15.0, 25.0),
//     Vec2::new(-25.0, 25.0),
// ];

// Bullet
pub const BULLET_SPEED: f32 = 20.0;
pub const BULLET_SPAWN_INTERVAL_DEFAULT: f32 = 0.10;
