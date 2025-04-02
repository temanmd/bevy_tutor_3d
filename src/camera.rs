use crate::character::{Character, STARTING_TRANSLATION};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 100.;

pub struct CameraPlugin;

#[derive(Component, Debug)]
struct OrbitCamera {
    target: Vec3,
    radius: f32,
    yaw: f32,
    pitch: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: STARTING_TRANSLATION,
            radius: CAMERA_DISTANCE,
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(Update, (move_camera_target, orbit_camera_system));
    }
}

#[derive(Component, Debug, Default)]
struct CameraTarget();

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((
            CameraTarget::default(),
            Transform::from_translation(STARTING_TRANSLATION),
        ))
        .with_child((
            Camera3d::default(),
            Transform::from_xyz(0., CAMERA_DISTANCE, -100.)
                .looking_at(STARTING_TRANSLATION, Vec3::Z),
            OrbitCamera::default(),
        ));
}

fn move_camera_target(
    mut query: Query<&mut Transform, With<CameraTarget>>,
    query_player: Query<&Transform, (With<Character>, Without<CameraTarget>)>,
) {
    let mut transform = query.single_mut();
    let character_transform = query_player.single();

    transform.translation = character_transform.translation;
}

fn orbit_camera_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut OrbitCamera), With<Camera3d>>,
) {
    if mouse_button_input.pressed(MouseButton::Right) {
        let sensitivity = 0.005;
        for event in mouse_motion_events.read() {
            let (mut transform, mut orbit) = query.single_mut();

            orbit.yaw -= event.delta.x * sensitivity;
            orbit.pitch -= event.delta.y * sensitivity;
            orbit.pitch = orbit.pitch.clamp(0.2, 1.4835);

            let offset = Vec3::new(
                orbit.radius * orbit.yaw.cos() * orbit.pitch.cos(),
                orbit.radius * orbit.pitch.sin(),
                orbit.radius * orbit.yaw.sin() * orbit.pitch.cos(),
            );

            transform.translation = orbit.target + offset;
            transform.look_at(orbit.target, Vec3::Y);
        }
    }
}
