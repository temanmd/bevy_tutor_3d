use crate::character::{CHARACTER_MODEL_PATH, MoveState};
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
        app.add_systems(Startup, (load_animation,));
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

    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: node_indices,
        graph: graph_handle,
    });
}

fn setup_scene_once_loaded(
    mut commands: Commands,
    animations: Res<Animations>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
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

        *done = true;
    }
}

fn animate_on_move(
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    move_states: Query<&MoveState, Changed<MoveState>>,
    animations: Res<Animations>,
) {
    if let Ok(move_state) = move_states.get_single() {
        for (mut player, mut transitions) in &mut animation_players {
            match *move_state {
                MoveState::Idle => {
                    transitions
                        .play(
                            &mut player,
                            animations.animations[0],
                            Duration::from_millis(150),
                        )
                        .set_speed(1.5)
                        .repeat();
                }
                MoveState::Run => {
                    transitions
                        .play(
                            &mut player,
                            animations.animations[1],
                            Duration::from_millis(150),
                        )
                        .set_speed(1.5)
                        .repeat();
                }
            }
        }
    }
}
