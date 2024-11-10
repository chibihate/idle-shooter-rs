use crate::{
    configs::{BACKGROUND_COLOR, BACKGROUND_SIZE},
    player::Player,
    state::GameState,
};
use bevy::{math::vec3, prelude::*};
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct FollowCameraPlugin;

impl Plugin for FollowCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(OnEnter(GameState::Loading), init_camera)
            .add_systems(
                Update,
                camera_follow_player.run_if(in_state(GameState::InGame)),
            );
    }
}

fn init_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgb_u8(BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2),
            custom_size: Some(BACKGROUND_SIZE),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // Centered at origin
        ..default()
    });
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single().translation;
    let (x, y) = (player_transform.x, player_transform.y);

    camera_transform.translation = camera_transform.translation.lerp(vec3(x, y, 0.0), 0.1);
}
