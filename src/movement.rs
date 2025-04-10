use crate::{
    camera::OrbitCamera,
    player::{MoveState, Player, Velocity},
};
use bevy::prelude::*;

pub const AVAILABLE_MOVE_KEYS: [KeyCode; 4] =
    [KeyCode::KeyW, KeyCode::KeyA, KeyCode::KeyS, KeyCode::KeyD];

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player_towards_camera);
    }
}

pub fn move_player_towards_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    camera_transform: Single<&Transform, (With<OrbitCamera>, Without<Player>)>,
    player: Single<(&mut Transform, &Velocity, &mut MoveState), With<Player>>,
    time: Res<Time>,
) {
    let (mut transform, velocity, mut move_state) = player.into_inner();

    if !keyboard_input.any_pressed(AVAILABLE_MOVE_KEYS) {
        move_state.set_if_neq(MoveState::Idle);
        return;
    }

    move_state.set_if_neq(MoveState::Run);

    let delta_secs = time.delta_secs();
    let mut movement = Vec3::default();
    let forward = camera_transform.forward().with_y(0.0).normalize_or_zero();

    let right = forward.cross(Vec3::Y).normalize();
    let left = -right;
    let back = -forward;

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement += forward;
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement += right;
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            movement += left;
        }
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        movement += back;
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement += right;
        } else if keyboard_input.pressed(KeyCode::KeyA) {
            movement += left;
        }
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        movement += left;
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement += forward;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            movement += back;
        }
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        movement += right;
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement += forward;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            movement += back;
        }
    }

    movement = movement.normalize_or_zero();
    let target = transform.translation + movement;

    let target_rotation = transform.looking_at(target, Vec3::Y).rotation;

    transform
        .rotation
        .smooth_nudge(&target_rotation, 10.0, delta_secs);

    transform.translation += movement * velocity.value * delta_secs;
}
