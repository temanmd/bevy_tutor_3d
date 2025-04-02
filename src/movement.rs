use crate::character::{Character, Velocity};
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

fn update_position(
    mut query: Query<(&Velocity, &mut Transform), With<Character>>,
    time: Res<Time>,
) {
    let (velocity, mut transform) = query.single_mut();
    transform.translation += velocity.value * time.delta_secs() * 5.;
}
