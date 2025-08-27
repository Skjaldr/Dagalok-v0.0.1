use bevy::{prelude::*, utils::HashMap};
use crate::asset_manager::GameAssets;
use super::AssetLoadingState;

#[derive(Resource, Debug, Component, Clone)]
pub struct GraphAnimations {
    pub index_node: HashMap<String, AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Resource, Debug, Component, Clone, Default)]
pub struct Animations {
    pub run: AnimationNodeIndex,
    pub idle: AnimationNodeIndex,
    pub idle_sword_shield: AnimationNodeIndex,
    pub run_sword_shield: AnimationNodeIndex,
    pub attack_sword: AnimationNodeIndex,

}

impl Animations {
    pub fn new(node: GraphAnimations) -> Self {
        Self {
            run: node.index_node.get("Run_Standard").expect("Failed to get run animation").clone(),
            idle: node.index_node.get("Idle").expect("Failed to get idle animation").clone(),
            idle_sword_shield: node.index_node.get("Idle_Sword_And_Shield").expect("Failed to get Idle_Sword_Shield Animation").clone(),
            run_sword_shield: node.index_node.get("Run_Sword_And_Shield").expect("Failed to get Run_Sword_Shield Animation").clone(),
            attack_sword: node.index_node.get("Attack_Sword_And_Shield_Slash").expect("Failed to get Sword Attack animation").clone(),
        }
    }
}

#[derive(Component, Resource)]
pub struct AnimationController {
    pub animations: GraphAnimations,
    pub _current: Option<usize>,
}

impl AnimationController {
    pub fn new(animations: GraphAnimations) -> Self {
        Self {
            animations,
            _current: None,
        }
    }
}


pub fn get_animations(
    mut commands: Commands,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    ga: Res<GameAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<AssetLoadingState>>,
) {

    let mut animations: HashMap<String, Handle<AnimationClip>> = HashMap::new();
    //let mut list = HashMap::new();
    let mut animation_graph = AnimationGraph::new();

    for (_, handle) in &ga.source {
        if let Some(gltf) = gltf_assets.get(handle) {
            for named_animation in gltf.named_animations.iter() {
                println!("Inserting animations: {}", named_animation.0);
               
                animations.insert(
                    named_animation.0.clone().to_string(),
                    gltf.named_animations[named_animation.0].clone()
                );
            }
        }
    }
    let mut indices = HashMap::new();

    for (name, clip) in animations.iter() {
        let new_clip = animation_graph.add_clip(
            clip.clone(), 
            1.0,
            animation_graph.root
        );
       
        indices.insert(name.clone(), new_clip);
    }
    
    let graph = graphs.add(animation_graph.clone());
    let anim_graph = GraphAnimations {
        index_node: indices.clone(),
        graph: graph.clone()
    };

    commands.insert_resource(anim_graph);
    next_state.set(AssetLoadingState::Spawning);
}

