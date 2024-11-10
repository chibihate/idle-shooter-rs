use std::f32::consts::PI;
use std::time::Duration;

use crate::components::Health;
use crate::configs::*;
use crate::player::Player;
use crate::{resources::GlobalTextureAtlas, state::GameState};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub enum EnemyType {
    Green,
    Red,
    Skin,
}

impl EnemyType {
    fn get_rand_enemy() -> Self {
        let mut rng = rand::thread_rng();
        let rand_index = rng.gen_range(0..3);
        return match rand_index {
            0 => Self::Green,
            1 => Self::Red,
            _ => Self::Skin,
        };
    }

    pub fn get_base_sprite_index(&self) -> usize {
        match self {
            EnemyType::Green => 8,
            EnemyType::Red => 12,
            EnemyType::Skin => 20,
        }
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_enemies.run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_INTERVAL))),
                update_enemy_transform,
                despawn_enemies,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_NUM_ENEMIES - num_enemies).min(SPAWN_RATE_PER_SECOND);

    if num_enemies >= MAX_NUM_ENEMIES || player_query.is_empty() {
        return;
    }

    let player_position = player_query.single().translation.truncate();
    for _ in 0..enemy_spawn_count {
        let position = get_random_position_around(player_position);
        let enemy_type = EnemyType::get_rand_enemy();
        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform {
                    translation: vec3(position.x, position.y, 1.0),
                    scale: Vec3::splat(SPRITE_SCALE_FACTOR),
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: enemy_type.get_base_sprite_index(),
            },
            Enemy,
            Health { value: 100.0 },
        ));
    }
}

fn get_random_position_around(player_position: Vec2) -> Vec2 {
    let mut rng = rand::thread_rng();
    let offset_x = (BACKGROUND_SIZE.x - BACKGROUND_OFFSET.x) / 2.0;
    let offset_y = (BACKGROUND_SIZE.y - BACKGROUND_OFFSET.y) / 2.0;
    let mut enemy_position = Vec2::ZERO;
    let mut distance = 0.0;
    while distance < 200.0 {
        let angle = rng.gen_range(0.0..PI * 2.0);
        let random_x = rng.gen_range(-offset_x..offset_x);
        let random_y = rng.gen_range(-offset_y..offset_y);
        enemy_position = Vec2::new(angle.cos() * random_x, angle.sin() * random_y);
        distance = player_position.distance(enemy_position);
    }
    enemy_position
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    let player_pos = player_query.single().translation;
    for mut transform in enemy_query.iter_mut() {
        let dir = (player_pos - transform.translation).normalize();
        transform.translation += dir * ENEMY_SPEED;
    }
}

fn despawn_enemies(mut commands: Commands, enemy_query: Query<(Entity, &Health), With<Enemy>>) {
    if enemy_query.is_empty() {
        return;
    }

    for (entity, health) in enemy_query.iter() {
        if health.value <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
