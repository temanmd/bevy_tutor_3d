use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Floor;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
    }
}

fn spawn_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_translation(Vec3::ZERO).with_scale(Vec3::new(20., 10., 20.)),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("floor_green.glb"))),
        Floor::default(),
    ));
}
