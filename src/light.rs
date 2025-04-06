use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            color: Color::srgba(1.0, 0.95, 0.8, 1.0),
            brightness: 1500.0,
        });
        app.add_systems(Startup, add_light);
    }
}

fn add_light(mut commands: Commands) {
    commands.spawn((
        Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            1.0,
            -std::f32::consts::PI / 4.,
        )),
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.,
            ..default()
        },
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 200.0,
            maximum_distance: 400.0,
            ..default()
        }
        .build(),
    ));
}
