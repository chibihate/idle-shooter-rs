use bevy::{ecs::observer::TriggerTargets, prelude::*};

use crate::{
    gun::Bullet,
    player::{NearestEnemy, Player},
    resources::GlobalTextureAtlas,
    state::GameState,
    BULLET_SPEED, SPRITE_SCALE_FACTOR,
};

use kdtree::distance::squared_euclidean;
use kdtree::KdTree;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

#[derive(Resource)]
pub struct EnemyKdTree {
    pub tree: KdTree<f32, Entity, [f32; 2]>,
}

impl Default for EnemyKdTree {
    fn default() -> Self {
        Self {
            tree: KdTree::new(2),
        }
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyKdTree>()
            .add_systems(OnEnter(GameState::GameInit), spawn_enemies)
            .add_systems(
                Update,
                (update_kdtree, find_nearest_enemy, handle_bullet_collision),
            );
    }
}

fn spawn_enemies(mut commands: Commands, handle: Res<GlobalTextureAtlas>) {
    commands.spawn((
        SpriteBundle {
            texture: handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 8,
        },
        Enemy,
    ));
}

fn update_kdtree(
    mut tree: ResMut<EnemyKdTree>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    if enemy_query.is_empty() {
        return;
    }
    // Clear and rebuild tree each frame
    tree.tree = KdTree::new(2);

    for (entity, transform) in enemy_query.iter() {
        let position = transform.translation;
        let point = [position.x, position.y]; // Only using x and y coordinates
        let _ = tree.tree.add(point, entity);
    }
}

fn find_nearest_enemy(
    tree: Res<EnemyKdTree>,
    mut player_query: Query<(&Transform, &mut NearestEnemy), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    for (player_transform, mut nearest) in player_query.iter_mut() {
        let player_pos = player_transform.translation;
        let search_point = [player_pos.x, player_pos.y];

        // Find nearest enemy
        if let Ok(nearest_results) = tree.tree.nearest(&search_point, 1, &squared_euclidean) {
            if let Some((distance, &entity)) = nearest_results.first() {
                nearest.entity = Some(entity);
                nearest.distance = distance.sqrt();
            } else {
                // No enemies found
                nearest.entity = None;
                nearest.distance = f32::MAX;
            }
        }
    }
}

fn handle_bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    enemy_kdtree: Res<EnemyKdTree>,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (bullet_entity, bullet_transform, bullet) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation;
        let search_point = [bullet_pos.x, bullet_pos.y];

        // Find nearest enemy
        if let Ok(nearest_results) = enemy_kdtree
            .tree
            .nearest(&search_point, 1, &squared_euclidean)
        {
            if let Some((distance, &entity)) = nearest_results.first() {
                if distance.sqrt() <= BULLET_SPEED {
                    // Collision range
                    // Handle collision
                    if let Ok((_, _)) = enemy_query.get_mut(entity) {
                        // enemy_health.current -= bullet.damage;, &mut Health

                        // // Remove bullet
                        // if commands.get_entity(bullet_entity).is_none() {
                        //     continue;
                        // };
                        // info!("Hit");
                        commands.entity(bullet_entity).despawn();
                    }
                }
            }
        }
    }
}
