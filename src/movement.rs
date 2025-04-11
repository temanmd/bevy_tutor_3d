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
                (
                    move_player_direction_controller,
                    move_player_animation_controller,
                ),
                move_rotate_player,
            )
                .chain(),
        );
        app.add_observer(on_player_move);
    }
}

#[derive(Event)]
struct PlayerMove {
    direction: Vec3,
}

fn move_player_animation_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut MoveState, With<Player>>,
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

    let mut input_direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyW) {
        input_direction.z += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input_direction.z -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input_direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input_direction.x += 1.0;
    }

    commands.trigger(PlayerMove {
        direction: input_direction.normalize_or_zero(),
    });
}

fn on_player_move(
    trigger: Trigger<PlayerMove>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    camera_query: Query<&Transform, (With<OrbitCamera>, Without<Player>)>,
) {
    if let Ok(mut player_data) = player_query.get_single_mut() {
        if let Ok(camera_transform) = camera_query.get_single() {
            let direction = &trigger.direction;
            let forward = camera_transform.forward().with_y(0.0).normalize_or_zero();
            let right = forward.cross(Vec3::Y).normalize();
            let world_direction = forward * direction.z + right * direction.x;

            player_data.target = Some(world_direction);
        }
    }
}

pub fn move_rotate_player(
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
