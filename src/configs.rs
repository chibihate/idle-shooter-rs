use bevy::prelude::Vec2;

// Window
pub const WINDOW_SIZE: Vec2 = Vec2::new(720.0, 480.0);

// Sprites
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const SPRITE_SCALE_FACTOR: f32 = 3.0;
pub const TILE_WIDTH: u32 = 16;
pub const TILE_HEIGHT: u32 = 16;
pub const SPRITE_SHEET_WIDTH: u32 = 8;
pub const SPRITE_SHEET_HEIGHT: u32 = 8;

// Player
pub const MINIMUM_DISTANCE_CURSOR_PLAYER: f32 = 50.0;

// Enemy
pub const MAX_NUM_ENEMIES: usize = 500;
pub const ENEMY_DAMAGE: f32 = 1.0;
pub const SPAWN_RATE_PER_SECOND: usize = 5;
pub const ENEMY_HEALTH: f32 = 100.0;
pub const ENEMY_SPAWN_INTERVAL: f32 = 1.0;
pub const ENEMY_SPEED: f32 = 1.0;

// Colors
pub const BACKGROUND_SIZE: Vec2 = Vec2::new(1920.0, 1080.0);
pub const BACKGROUND_OFFSET: Vec2 = Vec2::new(140.0, 70.0);
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

// Bullet
pub const BULLET_SPEED: f32 = 20.0;
pub const BULLET_SPAWN_INTERVAL_DEFAULT: f32 = 0.10;
pub const BULLET_RANGE: f32 = 200.0;
pub const BULLET_DAMAGE: f32 = 20.0;
pub const BULLET_PIERCES: u32 = 1;
pub const BULLETS_PER_SHOT: usize = 1;
