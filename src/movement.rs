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
        app.add_systems(
            Update,
            (
                move_player_towards_camera,
                move_player_direction_controller,
                move_player_animation_controller,
            ),
        );
        app.add_observer(on_player_move);
    }
}

enum MoveDirection {
    Left,
    Right,
    Forward,
    Backward,
    // LeftForward,
    // LeftBackward,
    // RightForward,
    // RightBackward,
}

#[derive(Event)]
struct PlayerMove {
    direction: MoveDirection,
}

fn move_player_animation_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut MoveState, With<Player>>,
    mut commands: Commands,
) {
    if let Ok(mut player_move_state) = player_query.get_single_mut() {
        if keyboard_input.any_just_pressed(AVAILABLE_MOVE_KEYS) {
            player_move_state.set_if_neq(MoveState::Run);
        } else if keyboard_input.any_just_released(AVAILABLE_MOVE_KEYS)
            && !keyboard_input.any_pressed(AVAILABLE_MOVE_KEYS)
        {
            player_move_state.set_if_neq(MoveState::Idle);
        }
    }
}

fn move_player_direction_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
) {
    if !keyboard_input.any_pressed(AVAILABLE_MOVE_KEYS) {
        return;
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        commands.trigger(PlayerMove {
            direction: MoveDirection::Forward,
        });
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        commands.trigger(PlayerMove {
            direction: MoveDirection::Backward,
        });
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        commands.trigger(PlayerMove {
            direction: MoveDirection::Left,
        });
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        commands.trigger(PlayerMove {
            direction: MoveDirection::Right,
        });
    }
}

fn on_player_move(
    trigger: Trigger<PlayerMove>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    camera_query: Query<&Transform, (With<OrbitCamera>, Without<Player>)>,
) {
    if let Ok(mut player_data) = player_query.get_single_mut() {
        if let Ok(camera_transform) = camera_query.get_single() {
            let direction = &trigger.direction;
            let mut movement = Vec3::default();
            let forward = camera_transform.forward().with_y(0.0).normalize_or_zero();
            let right = forward.cross(Vec3::Y).normalize();
            let left = -right;
            let backward = -forward;

            match direction {
                MoveDirection::Forward => movement += forward,
                MoveDirection::Backward => movement += backward,
                MoveDirection::Left => movement += left,
                MoveDirection::Right => movement += right,
            }

            movement = movement.normalize_or_zero();

            player_data.target = Some(movement);
        }
    }
}

pub fn move_player_towards_camera(
    mut player_data: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_data) = player_data.get_single_mut() {
        if player_data.1.target.is_some() {
            let (mut transform, mut velocity) = player_data;

            let delta_secs = time.delta_secs();
            let move_target = velocity.target.unwrap();
            let target = transform.translation + move_target;
            let target_rotation = transform.looking_at(target, Vec3::Y).rotation;

            transform
                .rotation
                .smooth_nudge(&target_rotation, 10.0, delta_secs);

            transform.translation += move_target * velocity.value * delta_secs;

            velocity.target = None;
        }
    }
}
