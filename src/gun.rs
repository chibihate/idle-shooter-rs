use std::u32;

use crate::components::{
    AttackSpeed, AttackSpeedPercent, Damage, Direction, GunBag, NearestEnemy, Pierces, Position,
    Range,
};
use crate::configs::*;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::resources::GlobalTextureAtlas;
use crate::state::GameState;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use rand::Rng;

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;
#[derive(Component)]
pub struct Bullet;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_gun,
                update_gun_transform,
                spawn_bullet,
                update_bullets,
                despawn_bullets,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn spawn_gun(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<(&AttackSpeedPercent, &Damage, &Range, &Pierces, &GunBag), With<Player>>,
    gun_query: Query<&Transform, With<Gun>>,
) {
    if player_query.is_empty() {
        return;
    }

    let num_guns = gun_query.iter().len() as u32;
    let (attack_speed, damage, range, pierces, gun_bag) = player_query.single();

    if gun_bag.size <= gun_bag.capacity && num_guns < gun_bag.size {
        let denominator = 100.0;
        let numerator = BULLET_SPAWN_INTERVAL_DEFAULT * denominator;
        let mut attack_speed_interval = attack_speed.value;
        if attack_speed_interval >= 0.0 {
            attack_speed_interval = numerator / (denominator + attack_speed_interval);
        } else {
            attack_speed_interval = (numerator - attack_speed_interval / numerator) / denominator;
        }

        commands.spawn((
            SpriteBundle {
                texture: handle.image.clone().unwrap(),
                transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            TextureAtlas {
                layout: handle.layout.clone().unwrap(),
                index: 17,
            },
            Gun,
            AttackSpeed {
                timer: Stopwatch::new(),
                interval: attack_speed_interval,
            },
            Range {
                value: BULLET_RANGE + range.value,
            },
            Damage {
                value: BULLET_DAMAGE + damage.value,
            },
            Pierces {
                value: BULLET_PIERCES + pierces.value,
            },
        ));
    }
}

fn update_gun_transform(
    player_query: Query<(&Transform, &NearestEnemy), (With<Player>, Without<Gun>, Without<Enemy>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>, Without<Gun>)>,
    mut gun_query: Query<
        (&mut Transform, &mut Sprite, &Range),
        (With<Gun>, Without<Player>, Without<Enemy>),
    >,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let (player_transform, nearest_enemy) = player_query.single();
    let player_position = player_transform.translation.truncate();

    for (index, (mut transform, mut sprite, range)) in gun_query.iter_mut().enumerate() {
        let default_rotation = Quat::from_rotation_z(0.0);
        let default_gun_position = player_position + GUN_OFFSET[index];

        if let Some(enemy_entity) = nearest_enemy.entity {
            if let Ok(enemy_transform) = enemy_query.get(enemy_entity) {
                if range.value >= nearest_enemy.distance {
                    let enemy_position = enemy_transform.translation.truncate();
                    let offset = 20.0;
                    let gun_offset = if GUN_OFFSET[index].x > 0.0 {
                        -offset
                    } else {
                        offset
                    };
                    let gun_position = player_position + GUN_OFFSET[index] + vec2(gun_offset, 0.0);
                    let angle = (enemy_position.y - gun_position.y)
                        .atan2(enemy_position.x - gun_position.x);
                    let new_gun_pos = vec2(
                        gun_position.x + offset * angle.cos(),
                        gun_position.y + offset * angle.sin(),
                    );
                    transform.rotation = Quat::from_rotation_z(angle);
                    transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, 15.0);
                    sprite.flip_x = false;
                    sprite.flip_y = player_position.x > enemy_position.x;
                    continue;
                }
            }
        }

        // Set default position and properties if enemy is out of range or not found
        transform.rotation = default_rotation;
        transform.translation = vec3(default_gun_position.x, default_gun_position.y, 15.0);
        sprite.flip_x = index % 2 != 0;
        sprite.flip_y = false;
    }
}

fn spawn_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut AttackSpeed, &Range, &Damage, &Pierces), With<Gun>>,
    handle: Res<GlobalTextureAtlas>,
) {
    if gun_query.is_empty() {
        return;
    }

    for (transform, mut attack_speed, range, damage, pierces) in gun_query.iter_mut() {
        let gun_rotation = transform.rotation;
        if gun_rotation.z == 0.0 {
            break;
        }
        let gun_position = transform.translation.truncate();
        attack_speed.timer.tick(time.delta());

        if attack_speed.timer.elapsed_secs() >= attack_speed.interval {
            attack_speed.timer.reset();
            let mut rng = rand::thread_rng();

            for _ in 0..BULLETS_PER_SHOT {
                let bullet_direction = transform.local_x();
                let direction = vec3(
                    bullet_direction.x + rng.gen_range(-0.5..0.5) * 0.0,
                    bullet_direction.y + rng.gen_range(-0.5..0.5) * 0.0,
                    bullet_direction.z,
                );
                commands.spawn((
                    SpriteBundle {
                        texture: handle.image.clone().unwrap(),
                        transform: Transform {
                            translation: vec3(gun_position.x, gun_position.y, 1.0),
                            rotation: gun_rotation,
                            scale: Vec3::splat(SPRITE_SCALE_FACTOR),
                        },
                        ..default()
                    },
                    TextureAtlas {
                        layout: handle.layout.clone().unwrap(),
                        index: 16,
                    },
                    Bullet,
                    Direction { value: direction },
                    Range { value: range.value },
                    Position {
                        value: Vec2 {
                            x: gun_position.x,
                            y: gun_position.y,
                        },
                    },
                    Damage {
                        value: damage.value,
                    },
                    Pierces {
                        value: pierces.value,
                    },
                ));
            }
        }
    }
}

fn update_bullets(mut bullet_query: Query<(&mut Transform, &Direction), With<Bullet>>) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut transform, direction) in bullet_query.iter_mut() {
        transform.translation += direction.value.normalize() * Vec3::splat(BULLET_SPEED);
        transform.translation.z = 10.0;
    }
}

fn despawn_bullets(
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &Transform, &Position, &Range, &mut Pierces), With<Bullet>>,
) {
    for (entity, transform, origin_position, range, mut pierces) in bullet_query.iter_mut() {
        let bullet_position = transform.translation;
        let distance = bullet_position
            .truncate()
            .distance(origin_position.value)
            .ceil();
        if distance >= range.value {
            pierces.value = 0;
        }

        if pierces.value == 0 {
            commands.entity(entity).despawn();
        }
    }
}
