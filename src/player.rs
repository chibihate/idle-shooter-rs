use crate::*;
use bevy::{math::vec3, prelude::*};
use bevy_egui::{egui, EguiContexts};
use resources::CursorPosition;

use crate::{resources::GlobalTextureAtlas, state::GameState};

pub struct PlayerPlugin;
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Speed {
    value: f32,
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
        Speed { value: 2.5 },
        GunBag {
            size: 1,
            capacity: 6,
        },
        NearestEnemy::default(),
    ));

    next_state.set(GameState::InGame);
}

fn update_player_movement(
    cursor_pos: Res<CursorPosition>,
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }

    let (mut player_transform, speed) = player_query.single_mut();
    let player_pos = player_transform.translation.truncate();
    let cursor_pos = match cursor_pos.0 {
        Some(pos) => pos,
        None => player_pos,
    };

    let distance = cursor_pos.distance(player_pos);

    if distance > MINIMUM_DISTANCE_CURSOR_PLAYER {
        let dir = (cursor_pos - player_pos).normalize();
        player_transform.translation += vec3(dir.x, dir.y, 0.0) * speed.value;
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
