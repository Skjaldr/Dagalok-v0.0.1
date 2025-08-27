use bevy::{math::quat, prelude::*, utils::HashMap};
use bevy_rapier3d::prelude::{Ccd, Collider, CollisionGroups, Group, LockedAxes, RigidBody};
use leafwing_input_manager::InputManagerBundle;
use spacetimedb_sdk::identity::Identity;


use crate::{animations::{run_animations, setup_animations::AnimationController}, asset_manager::GameAssets, input::GameActions, module_bindings::{EntityComponent, PlayerComponent}, network::player_connected::PlayerDataResource};

const PLAYER_GROUP: u32 = 0b01;
const ENVIRONMENT_GROUP: u32 = 0b10;

#[derive(Component, Debug, Clone)]
// pub struct Player  {
//     pub data: PlayerComponent,
// }
pub struct Player {
    pub data: PlayerComponent,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Component, Default)]
pub enum PlayerStances {
    #[default]
    NonCombat,
    Combat, // Normal combat state
    // Precise,
    // Defensive,
}

#[derive(Component)]
pub struct PlayerMark;

#[derive(Component, Debug, Clone)]
pub struct PlayerEntity {
    pub data: EntityComponent,
}

#[derive(Component, Clone)]
pub struct PlayerSceneHandle {
    pub player_scene: Handle<Scene>,
    //pub player_id: String,
}

impl PlayerSceneHandle {
    pub fn new(
        ga: &Res<GameAssets>,
        gltf_assets: &Res<Assets<Gltf>>,
    ) -> Self {
        let mut player_entity: Vec<Handle<Scene>> = Vec::new();
        //let mut id = String::new();
        for (_, handle) in &ga.gltf_files {
            if let Some(gltf) = gltf_assets.get(handle) {
                let scene0 = gltf.named_scenes["Scene"].clone();
                player_entity.push(scene0);
                //id = player_id.to_string();
            }
        }
        Self {
            player_scene: player_entity[0].clone(),
            //player_id: id,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player_scene: SceneBundle,
    pub player: Player,
    pub player_entity: PlayerEntity,
    pub collider: Collider,
    pub collision_group: CollisionGroups,
    pub body_type: RigidBody,
    pub ccd: Ccd,
    pub locked_axes: LockedAxes,
    pub stance: PlayerStances,

    
}

impl PlayerBundle {
    pub fn new(
        player: &Player,
        player_entity: &PlayerEntity,
        ga: &Res<GameAssets>,
        gltf_assets: &Res<Assets<Gltf>>,
    ) -> Self {
        let player_scene = PlayerSceneHandle::new(ga, gltf_assets);
        
        Self {
            player_scene: SceneBundle {
                scene: player_scene.player_scene.clone(),
                transform: Transform::from_xyz(0.0, 0.245, 0.0),
                ..Default::default()
            },
            player: Player {
                data: player.data.clone()
            },
            player_entity: player_entity.clone(),
            collider: Collider::capsule_y(0.40 / 2.0, 0.07 / 2.0), // Capsule collider with height and radius
            collision_group: CollisionGroups::new(Group::from_bits_truncate(PLAYER_GROUP), Group::from_bits_truncate(ENVIRONMENT_GROUP)),
            body_type: RigidBody::Dynamic,
            ccd: Ccd::enabled(),
            locked_axes: LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
            stance: PlayerStances::NonCombat,
            
            //anim_player: AnimationPlayer::default()
        }
    }
}
        
 