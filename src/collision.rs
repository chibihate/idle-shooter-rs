use bevy::prelude::*;
use kdtree::{distance::squared_euclidean, KdTree};

use crate::{
    components::{Damage, Health, NearestEnemy, Pierces},
    configs::*,
    enemy::Enemy,
    gun::Bullet,
    player::Player,
    state::GameState,
};

pub struct CollisionPlugin;

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
impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemyKdTree>().add_systems(
            Update,
            (
                update_enemy_kd_tree,
                find_nearest_enemy,
                handle_bullet_collision,
            )
                .run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_enemy_kd_tree(
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
    mut bullet_query: Query<(&Transform, &Damage, &mut Pierces), With<Bullet>>,
    enemy_kdtree: Res<EnemyKdTree>,
    mut enemy_query: Query<&mut Health, With<Enemy>>,
) {
    if bullet_query.is_empty() || enemy_query.is_empty() {
        return;
    }

    for (transform, damage, mut pierces) in bullet_query.iter_mut() {
        let bullet_position = transform.translation;
        let search_point = [bullet_position.x, bullet_position.y];

        // Find nearest enemy
        if let Ok(nearest_results) = enemy_kdtree
            .tree
            .nearest(&search_point, 1, &squared_euclidean)
        {
            if let Some((distance, &entity_enemy)) = nearest_results.first() {
                if distance.sqrt() <= BULLET_SPEED {
                    // Handle collision
                    if let Ok(mut heath) = enemy_query.get_mut(entity_enemy) {
                        heath.value -= damage.value;
                        pierces.value = pierces.value.saturating_sub(1);
                    }
                }
            }
        }
    }
}
