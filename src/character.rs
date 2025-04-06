use bevy::prelude::*;

pub const STARTING_TRANSLATION: Vec3 = Vec3::new(0., 0., 0.);
pub const STARTING_VELOCITY: f32 = 40.;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: f32,
}

#[derive(Component)]
pub struct Character;

#[derive(Resource)]
struct CharacterAnimations(Vec<Handle<AnimationClip>>);

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_character);
    }
}

fn run_animation(
    animations: Res<CharacterAnimations>,
    player: Single<&mut AnimationPlayer, With<Character>>,
) {
    // let (graph, mut animation) = AnimationGraph::from_clip(animations.0[0].clone_weak());
    // player.play(animation).repeat();
}

fn load_animation(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(CharacterAnimations(vec![
        asset_server.load(GltfAssetLabel::Animation(0).from_asset("character.glb")),
        asset_server.load(GltfAssetLabel::Animation(1).from_asset("character.glb")),
        asset_server.load(GltfAssetLabel::Animation(2).from_asset("character.glb")),
    ]));
}

fn spawn_character(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform {
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            scale: Vec3::splat(10.),
            translation: STARTING_TRANSLATION,
        },
        Velocity {
            value: STARTING_VELOCITY,
        },
        Character,
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("character.glb"))),
    ));
}
