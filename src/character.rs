use bevy::prelude::*;

pub const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., 0.);
pub const STARTING_VELOCITY: f32 = 70.;
pub const CHARACTER_MODEL_PATH: &str = "character.glb";

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: f32,
}

#[derive(Component, Default, Debug)]
pub struct Character {}

#[derive(Default, PartialEq, Component, Debug)]
pub enum MoveState {
    #[default]
    Idle,
    Run,
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character);
    }
}

pub fn spawn_character(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform {
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            scale: Vec3::splat(10.),
            translation: STARTING_TRANSLATION,
        },
        Velocity {
            value: STARTING_VELOCITY,
        },
        Character::default(),
        MoveState::default(),
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset(CHARACTER_MODEL_PATH))),
    ));
}
