use crate::components::{
    AttackSpeedPercent, Damage, GunBag, Health, NearestEnemy, Pierces, Range, Speed,
};
use crate::configs::*;
use crate::resources::CursorPosition;
use bevy::{math::vec3, prelude::*};
use bevy_egui::{egui, EguiContexts};

use crate::{resources::GlobalTextureAtlas, state::GameState};

pub struct PlayerPlugin;
#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), init_player)
            .add_systems(
                Update,
                update_player_movement.run_if(in_state(GameState::InGame)),
            )
            .add_systems(Update, player_ui);
    }
}

fn init_player(
    mut commands: Commands,
    handle: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: handle.image.clone().unwrap(),
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        TextureAtlas {
            layout: handle.layout.clone().unwrap(),
            index: 0,
        },
        Player,
        Health { value: 100.0 },
        Speed { value: 2.5 },
        AttackSpeedPercent { value: 0.0 },
        Damage { value: 0.0 },
        Range { value: 0.0 },
        Pierces { value: 0 },
        GunBag {
            size: 1,
            capacity: 6,
        },
        NearestEnemy::default(),
    ));

    next_state.set(GameState::InGame);
}

fn update_player_movement(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut player_transform, player_speed) = player_query.single_mut();
    let player_position = player_transform.translation.truncate();
    let cursor_position = match cursor_position.value {
        Some(position) => position,
        None => player_position,
    };

    let distance = cursor_position.distance(player_position);

    if distance > MINIMUM_DISTANCE_CURSOR_PLAYER {
        let direction = (cursor_position - player_position).normalize();
        player_transform.translation += vec3(direction.x, direction.y, 0.0) * player_speed.value;

        let extents = Vec3::from(((BACKGROUND_SIZE - BACKGROUND_OFFSET) / 2.0, 0.0));
        player_transform.translation = player_transform.translation.min(extents).max(-extents);
    }
}

fn player_ui(
    mut contexts: EguiContexts,
    mut player_query: Query<(&mut Speed, &mut GunBag), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut speed, mut gun_bag) = player_query.single_mut();

    egui::Window::new("Player").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Speed:");
            ui.add(egui::Slider::new(&mut speed.value, 0.0..=10.0).text("value"));
        });
        ui.horizontal(|ui| {
            ui.label("Gun:");
            ui.add(egui::Slider::new(&mut gun_bag.size, 1..=6).text("value"));
        });
    });
}
