use bevy::prelude::*;

#[derive(Component)]
pub struct Floor;

pub struct FloorPlugin;

impl Plugin for FloorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
    }
}

fn spawn_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let image = asset_server.load("textures/grass.png");

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(200., 200.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(image.clone()),
            ..default()
        })),
        Floor,
    ));
}
