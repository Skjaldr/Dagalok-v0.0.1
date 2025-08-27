use bevy::{prelude::*, utils::HashMap};
use bevy_third_person_camera::ThirdPersonCameraTarget;
use leafwing_input_manager::{prelude::{ActionState, InputMap}, InputManagerBundle};
use crate::{animations::{self, setup_animations::{AnimationController, Animations, GraphAnimations}}, asset_manager::GameAssets, input::GameActions, module_bindings::{create_player, entity_component, PlayerStances}, network::{player_connected::{NewPlayer, PlayerDataResource}, uncb_receiver::UncbEvent}};

use super::{player_bundle::{Player, PlayerBundle, PlayerEntity, PlayerMark, PlayerSceneHandle}, HandleScenesState};


#[derive(Resource, Debug)]
pub struct SceneEntitys(pub HashMap<String, Entity>);

#[derive(Component, Debug)]
pub struct SceneName(pub String);

#[derive(Resource, Debug, Clone)]
pub struct PlayerEntities(pub HashMap<String, Player>);

// Component tag for the main player
#[derive(Component)]
pub struct MainPlayer;


// Component tag for players who are not the main player
#[derive(Component)]
pub struct NonMainPlayer;
    
pub fn setup_player(
    commands: &mut Commands,
    player: &Player,
    player_entity: &PlayerEntity,
    is_main_player: bool,
    ga: &Res<GameAssets>,
    gltf_assets: &Res<Assets<Gltf>>,
    graph: &Res<GraphAnimations>,
    //assets: &Res<SceneAssets>,
    
) {
    
    let mut scene_entities: HashMap<String, Entity> = HashMap::new();
    let player_bundle = PlayerBundle::new(player, player_entity, ga, gltf_assets);


    if is_main_player {
        let mut player_commands = commands.spawn(player_bundle);
        player_commands
        .insert(ThirdPersonCameraTarget)
        .insert(AnimationController::new(GraphAnimations {
            index_node: graph.index_node.clone(),
            graph: graph.graph.clone(),
        }))
        .insert(PlayerMark)
        // .insert(controller)
        .insert(InputManagerBundle::<GameActions> {
            action_state: ActionState::default(),
            input_map: InputMap::new([
                (GameActions::Forward,  KeyCode::KeyE),
                (GameActions::Left,     KeyCode::KeyS),
                (GameActions::Right,    KeyCode::KeyF),
                (GameActions::Back,     KeyCode::KeyD),
                (GameActions::Jump,     KeyCode::Space),
                (GameActions::Crouch,   KeyCode::ControlLeft),
                (GameActions::CombatStance, KeyCode::KeyA),
                (GameActions::Attack, KeyCode::Digit1)
                ]),
            });
            let player_entity = player_commands.id();
            scene_entities.insert("Player".to_string(), player_entity);
            
        } else {
            println!("THIS IS A NON_MAIN_PLAYER!");
            let mut player_command = commands.spawn(player_bundle);
            let entity = player_command.id();
            player_command
            .insert(NonMainPlayer)
            .insert(AnimationController::new(GraphAnimations {
                index_node: graph.index_node.clone(),
                graph: graph.graph.clone(),
            }));
            
            scene_entities.insert("NonMainPlayer".to_string(), entity);
    }
    
    commands.insert_resource(SceneEntitys(scene_entities));
}

pub fn spawn_players(
    mut commands: Commands,
    ga: Res<GameAssets>,
    mut player_data: ResMut<PlayerDataResource>,
    assets_gltf: Res<Assets<Gltf>>,
    query: Query<&Player>,
    mut next_state: ResMut<NextState<HandleScenesState>>,
    graph: Res<GraphAnimations>,
) {

    for player_component in player_data.player_vec.iter_mut() {
       
        let player_exists = query.iter().any(|p: &Player| p.data.entity_id == player_component.data.entity_id);
        
        if !player_exists {
            if let Some(player_entity_data) = entity_component::EntityComponent::filter_by_entity_id(player_component.data.entity_id).next() {
                let player_entity = PlayerEntity {
                    data: player_entity_data.clone()
                };

                println!("Spawning player {:?} at {:?}", player_component.data.entity_id, player_entity.data.position);

                let main_player_bool = player_component.data.owner_id == spacetimedb_sdk::identity::identity().unwrap();

                println!("IS THIS THE MAIN PLAYER? {:?}", main_player_bool);
                setup_player(
                    &mut commands,
                    &player_component.clone(),
                    &player_entity,
                    main_player_bool,
                    &ga,
                    &assets_gltf,
                    &graph
                );

                next_state.set(HandleScenesState::Spawned);
            }
        }
    }
}

pub fn spawn_new_players(
    commands: Commands,
    player_data: ResMut<PlayerDataResource>,
    ga: Res<GameAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    query: Query<&Player>,
    next_state: ResMut<NextState<HandleScenesState>>,
    new_pl: Res<NewPlayer>,
    graph: Res<GraphAnimations>,
) {

    if new_pl.0 {
        spawn_players(commands, ga, player_data, assets_gltf, query, next_state, graph);
    }
}