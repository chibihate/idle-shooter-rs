use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use camera::FollowCameraPlugin;
use collision::CollisionPlugin;
use configs::*;
use enemy::EnemyPlugin;
use gun::GunPlugin;
use player::PlayerPlugin;
use resources::ResourcesPlugin;
use shooter_rs::state::GameState;
use shooter_rs::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Shooter".into(),
                        resizable: true,
                        focused: true,
                        resolution: WINDOW_SIZE.into(),
                        ..default()
                    }),
                    ..default()
                }),
            EguiPlugin,
        ))
        .init_state::<GameState>()
        // .insert_resource(ClearColor(Color::srgb_u8(
        //     BACKGROUND_COLOR.0,
        //     BACKGROUND_COLOR.1,
        //     BACKGROUND_COLOR.2,
        // )))
        .add_plugins(FollowCameraPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .insert_resource(Msaa::Off)
        .run();
}
