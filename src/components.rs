use bevy::{prelude::*, time::Stopwatch};

#[derive(Component)]
pub struct Speed {
    pub value: f32,
}
#[derive(Component)]
pub struct Health {
    pub value: f32,
}
#[derive(Component)]
pub struct GunBag {
    pub size: u32,
    pub capacity: u32,
}
#[derive(Component)]
pub struct NearestEnemy {
    pub entity: Option<Entity>,
    pub distance: f32,
}

impl Default for NearestEnemy {
    fn default() -> Self {
        Self {
            entity: None,
            distance: f32::MAX,
        }
    }
}

#[derive(Component)]
pub struct AttackSpeed {
    pub timer: Stopwatch,
    pub interval: f32,
}
#[derive(Component)]
pub struct AttackSpeedPercent {
    pub value: f32,
}

#[derive(Component)]
pub struct Damage {
    pub value: f32,
}

#[derive(Component)]
pub struct Range {
    pub value: f32,
}

#[derive(Component)]
pub struct Direction {
    pub value: Vec3,
}
#[derive(Component)]
pub struct Position {
    pub value: Vec2,
}
#[derive(Component)]
pub struct Pierces {
    pub value: u32,
}
