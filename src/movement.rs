use crate::{
    camera::OrbitCamera,
    character::{Character, Velocity},
};
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player_towards_camera);
    }
}

pub fn move_player_towards_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_transform: Single<&Transform, (With<OrbitCamera>, Without<Character>)>,
    player: Single<(&mut Transform, &Velocity), With<Character>>,
    time: Res<Time>,
) {
    if !keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD]) {
        return;
    }

    let (mut transform, velocity) = player.into_inner();
    let mut movement = Vec3::default();

    let forward = camera_transform.forward().with_y(0.0).normalize_or_zero();

    let right = forward.cross(Vec3::Y).normalize();
    let left = -right;
    let back = -forward;

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement += forward;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement += back;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        movement += left;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        movement += right;
    }

    movement = movement.clamp_length_max(1.0);
    let target = transform.translation + movement;

    transform.look_at(target, Vec3::Y);
    transform.translation += movement * velocity.value * time.delta_secs();
}
