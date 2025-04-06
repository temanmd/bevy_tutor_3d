use animation::AnimationPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use character::CharacterPlugin;
use debug::DebugPlugin;
use floor::FloorPlugin;
use light::LightPlugin;
use movement::MovementPlugin;

mod animation;
mod camera;
mod character;
mod debug;
mod floor;
mod light;
mod movement;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugins(CharacterPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(FloorPlugin)
        .add_plugins(AnimationPlugin)
        .run();
}
