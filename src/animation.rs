use crate::{character::CHARACTER_MODEL_PATH, movement::AVAILABLE_MOVE_KEYS};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct Animations {
    pub animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_animation);
        app.add_systems(Update, (setup_scene_once_loaded, animate_on_move));
    }
}

fn load_animation(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(8).from_asset(CHARACTER_MODEL_PATH)),
        asset_server.load(GltfAssetLabel::Animation(37).from_asset(CHARACTER_MODEL_PATH)),
    ]);

    // Insert a resource with the current scene information
    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: node_indices,
        graph: graph_handle,
    });
}

fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    graphs: Res<Assets<AnimationGraph>>,
    mut clips: ResMut<Assets<AnimationClip>>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    fn get_clip<'a>(
        node: AnimationNodeIndex,
        graph: &AnimationGraph,
        clips: &'a mut Assets<AnimationClip>,
    ) -> &'a mut AnimationClip {
        let node = graph.get(node).unwrap();
        let clip = match &node.node_type {
            AnimationNodeType::Clip(handle) => clips.get_mut(handle),
            _ => unreachable!(),
        };
        clip.unwrap()
    }

    if let Ok(player_data) = players.get_single_mut() {
        let (entity, mut player) = player_data;
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[0], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}

fn animate_on_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    animations: Res<Animations>,
) {
    if keyboard_input.any_just_pressed(AVAILABLE_MOVE_KEYS) {
        for (mut player, mut transitions) in &mut animation_players {
            let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
                continue;
            };

            transitions
                .play(
                    &mut player,
                    animations.animations[1],
                    Duration::from_millis(250),
                )
                .repeat();
        }
    } else if keyboard_input.any_just_released(AVAILABLE_MOVE_KEYS)
        && !keyboard_input.any_pressed(AVAILABLE_MOVE_KEYS)
    {
        for (mut player, mut transitions) in &mut animation_players {
            let Some((&playing_animation_index, _)) = player.playing_animations().next() else {
                continue;
            };

            transitions
                .play(
                    &mut player,
                    animations.animations[0],
                    Duration::from_millis(250),
                )
                .repeat();
        }
    }
}
