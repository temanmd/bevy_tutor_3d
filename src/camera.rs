use crate::character::Character;
use crate::movement::update_position;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

const DEFAULT_CAMERA_DISTANCE: f32 = 100.;
const TARGET_LOOK_AT_OFFSET: Vec3 = Vec3::new(0., 9., 0.);

pub struct CameraPlugin;

#[derive(Component, Debug)]
pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    radius: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.5,
            radius: DEFAULT_CAMERA_DISTANCE,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(
            Update,
            (zoom_camera, orbit_camera).chain().after(update_position),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), OrbitCamera::default()));
}

fn zoom_camera(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut orbit: Single<&mut OrbitCamera>,
) {
    for event in mouse_wheel_events.read() {
        let zoom_sensitivity = 3.0;
        orbit.radius = (orbit.radius - event.y * zoom_sensitivity).clamp(30.0, 150.0);
    }
}

fn orbit_camera(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut OrbitCamera), With<Camera3d>>,
    player_transform: Single<&Transform, (With<Character>, Without<Camera3d>)>,
) {
    let (mut transform, mut orbit) = query.single_mut();

    if mouse_button_input.pressed(MouseButton::Right) {
        let sensitivity = 0.005;

        for event in mouse_motion_events.read() {
            orbit.yaw -= event.delta.x * sensitivity;
            orbit.pitch += event.delta.y * sensitivity;
        }

        orbit.pitch = orbit.pitch.clamp(0.1, 1.4835);
    }

    orbit.yaw %= std::f32::consts::TAU;
    let rotation = Quat::from_euler(EulerRot::YXZ, orbit.yaw, orbit.pitch, 0.0);
    let offset = rotation * Vec3::new(0., 0., -orbit.radius);

    let player_translation = &player_transform.translation;
    let target_look_to = -offset + TARGET_LOOK_AT_OFFSET;

    transform.translation = player_translation + offset;
    transform.look_to(target_look_to, Vec3::Y);
}
