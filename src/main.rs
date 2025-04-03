use bevy::prelude::*;
use camera::CameraPlugin;
use character::CharacterPlugin;
use debug::DebugPlugin;
use floor::FloorPlugin;
use movement::MovementPlugin;

mod camera;
mod character;
mod debug;
mod floor;
mod movement;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1200.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CharacterPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(FloorPlugin)
        .run();
}
