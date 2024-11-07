use crate::player::{GunBag, Player};
use crate::resources::GlobalTextureAtlas;
use crate::*;
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use enemy::Enemy;
use player::NearestEnemy;

pub struct GunPlugin;

#[derive(Component)]
pub struct Gun;
#[derive(Component)]
pub struct GunTimer {
    timer: Stopwatch,
    interval: f32,
}
#[derive(Component)]
pub struct GunRange {
    value: f32,
}
#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
struct BulletDirection {
    value: Vec3,
}
#[derive(Component)]
pub struct BulletRange {
    value: f32,
    origin_position: Vec2,
}

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_gun, update_gun_transform))
            .add_systems(Update, (spawn_bullet, update_bullets));
    }
}

fn spawn_gun(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    player_query: Query<&GunBag, With<Player>>,
    gun_query: Query<&Transform, With<Gun>>,
) {
    if player_query.is_empty() {
        return;
    }

    let num_guns = gun_query.iter().len() as u32;
    let gun_bag = player_query.single();

    if gun_bag.size <= gun_bag.capacity && num_guns < gun_bag.size {
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
            GunTimer {
                timer: Stopwatch::new(),
                interval: BULLET_SPAWN_INTERVAL_DEFAULT,
            },
            GunRange { value: 200.0 },
        ));
    }
}

fn update_gun_transform(
    player_query: Query<(&Transform, &NearestEnemy), (With<Player>, Without<Gun>, Without<Enemy>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>, Without<Gun>)>,
    mut gun_query: Query<
        (&mut Transform, &mut Sprite, &GunRange),
        (With<Gun>, Without<Player>, Without<Enemy>),
    >,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    let (player_transform, nearest) = player_query.single();
    let player_pos = player_transform.translation.truncate();

    for (index, (mut transform, mut sprite, gun_range)) in gun_query.iter_mut().enumerate() {
        let default_rotation = Quat::from_rotation_z(0.0);
        let default_gun_pos = player_pos + GUN_OFFSET[index];

        if let Some(enemy_entity) = nearest.entity {
            if let Ok(enemy_transform) = enemy_query.get(enemy_entity) {
                if gun_range.value >= nearest.distance {
                    let enemy_pos = enemy_transform.translation.truncate();
                    let offset = 20.0;
                    let gun_offset = if GUN_OFFSET[index].x > 0.0 {
                        -offset
                    } else {
                        offset
                    };
                    let gun_pos = player_pos + GUN_OFFSET[index] + vec2(gun_offset, 0.0);
                    let angle = (enemy_pos.y - gun_pos.y).atan2(enemy_pos.x - gun_pos.x);
                    let new_gun_pos = vec2(
                        gun_pos.x + offset * angle.cos(),
                        gun_pos.y + offset * angle.sin(),
                    );
                    transform.rotation = Quat::from_rotation_z(angle);
                    transform.translation = vec3(new_gun_pos.x, new_gun_pos.y, 15.0);
                    sprite.flip_x = false;
                    sprite.flip_y = player_pos.x > enemy_pos.x;
                    continue;
                }
            }
        }

        // Set default position and properties if enemy is out of range or not found
        transform.rotation = default_rotation;
        transform.translation = vec3(default_gun_pos.x, default_gun_pos.y, 15.0);
        sprite.flip_x = index % 2 != 0;
        sprite.flip_y = false;
    }
}

fn spawn_bullet(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut GunTimer, &GunRange), With<Gun>>,
    handle: Res<GlobalTextureAtlas>,
) {
    if gun_query.is_empty() {
        return;
    }

    for (gun_transform, mut gun_timer, gun_range) in gun_query.iter_mut() {
        let gun_rotation = gun_transform.rotation;
        if gun_rotation.z == 0.0 {
            break;
        }
        let gun_pos = gun_transform.translation.truncate();
        gun_timer.timer.tick(time.delta());

        if gun_timer.timer.elapsed_secs() >= gun_timer.interval {
            gun_timer.timer.reset();
            let bullet_direction = gun_transform.local_x();
            let direction = vec3(bullet_direction.x, bullet_direction.y, bullet_direction.z);
            commands.spawn((
                SpriteBundle {
                    texture: handle.image.clone().unwrap(),
                    transform: Transform {
                        translation: vec3(gun_pos.x, gun_pos.y, 1.0),
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
                BulletDirection { value: direction },
                BulletRange {
                    value: gun_range.value,
                    origin_position: Vec2 {
                        x: gun_pos.x,
                        y: gun_pos.y,
                    },
                },
            ));
        }
    }
}

fn update_bullets(mut bullet_query: Query<(&mut Transform, &BulletDirection), With<Bullet>>) {
    if bullet_query.is_empty() {
        return;
    }

    for (mut t, dir) in bullet_query.iter_mut() {
        t.translation += dir.value.normalize() * Vec3::splat(BULLET_SPEED);
        t.translation.z = 10.0;
    }
}

fn despawn_old_bullets(
    mut commands: Commands,
    bullet_query: Query<(&Transform, &BulletRange, Entity), With<Bullet>>,
) {
    if bullet_query.is_empty() {
        return;
    }

    for (transform, range, e) in bullet_query.iter() {
        let distance = transform
            .translation
            .truncate()
            .distance(range.origin_position)
            .ceil();
        if distance >= range.value {
            if commands.get_entity(e).is_none() {
                continue;
            };
            info!("distance");
            commands.entity(e).despawn();
            // commands.entity(e).despawn();
        }
    }
}
