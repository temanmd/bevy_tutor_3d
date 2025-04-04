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
    camera: Single<&OrbitCamera>,
    player: Single<(&mut Transform, &Velocity), With<Character>>,
    time: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::KeyW) {
        let (mut transform, velocity) = player.into_inner();

        let forward = &camera.normalized_forward;
        let target = transform.translation + forward;
        transform.look_at(target, Vec3::Y);
        transform.translation += forward * velocity.value * time.delta_secs();
    }
}
