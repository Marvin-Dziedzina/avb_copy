use bevy::prelude::*;

use crate::states::AppState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(AppState::MainMenu), setup);
    }
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    log::info!("Setting up scene");

    let scene: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("empty_plane_50x50m.glb"));
    commands.spawn((
        Name::new("TestScene"),
        SceneRoot(scene),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
        children![(
            Camera3d::default(),
            Transform::from_xyz(0.0, 20.0, 50.0).looking_at(Vec3::splat(0.0), Vec3::Y),
            children![(
                PointLight {
                    intensity: 800000.0,
                    shadows_enabled: true,
                    ..Default::default()
                },
                Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::splat(0.0), Vec3::Y),
            )]
        ),],
    ));
}
