use crate::movement::move_rotate_player;
use crate::player::Player;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

const DEFAULT_CAMERA_DISTANCE: f32 = 100.;
const TARGET_LOOK_TO_OFFSET: Vec3 = Vec3::new(0., 20., 0.);
const ZOOM_SENSITIVITY: f32 = 3.0;
const CAMERA_ORBIT_SENSITIVITY: f32 = 0.005;

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
            (
                (zoom_camera, orbit_camera)
                    .chain()
                    .after(move_rotate_player),
                orbit_camera_mouse_controller,
            ),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), OrbitCamera::default()));
}

fn zoom_camera(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut orbit_query: Query<&mut OrbitCamera>,
) {
    if let Ok(mut orbit_camera) = orbit_query.get_single_mut() {
        for event in mouse_wheel_events.read() {
            orbit_camera.radius = event
                .y
                .mul_add(-ZOOM_SENSITIVITY, orbit_camera.radius)
                .clamp(70.0, 200.0);
        }
    }
}

fn orbit_camera_mouse_controller(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut window: Single<&mut Window>,
) {
    if mouse_button_input.pressed(MouseButton::Right) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    } else if mouse_button_input.just_released(MouseButton::Right) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
    }
}

fn orbit_camera(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut OrbitCamera), With<Camera3d>>,
    player_transform: Single<&Transform, (With<Player>, Without<Camera3d>)>,
) {
    let (mut transform, mut orbit) = query.single_mut();

    if mouse_button_input.pressed(MouseButton::Right) {
        let sensitivity = CAMERA_ORBIT_SENSITIVITY;

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
    let target_look_to = -offset + TARGET_LOOK_TO_OFFSET;

    transform.translation = player_translation + offset;
    transform.look_to(target_look_to, Vec3::Y);
}
