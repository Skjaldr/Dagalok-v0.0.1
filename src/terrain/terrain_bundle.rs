use bevy::prelude::*;
use bevy_rapier3d::prelude::{CollisionGroups, Group};


use crate::asset_manager::GameAssets;

const PLAYER_GROUP: u32 = 0b01;
const ENVIRONMENT_GROUP: u32 = 0b10;

#[derive(Resource, Default)]
pub struct TerrainSceneHandle {
    pub terrain_scene: Handle<Scene>,
    //pub player_id: String,
}

impl TerrainSceneHandle {
    pub fn new(
        ga: &Res<GameAssets>,
        gltf_assets: &Res<Assets<Gltf>>,
    ) -> Self {
        let mut terrain_entity: Vec<Handle<Scene>> = Vec::new();
        //let mut id = String::new();
        for (_, handle) in &ga.terrain_files {
            if let Some(gltf) = gltf_assets.get(handle) {
                let scene0 = gltf.scenes[0].clone();
                terrain_entity.push(scene0);
            }
        }
        Self {
            terrain_scene: terrain_entity[0].clone(),
    
        }
    }
}

#[derive(Component)]
pub struct TerrainMarker;

#[derive(Bundle)]
pub struct TerrainBundle {
    pub terrain_scene: SceneBundle,
    pub collision_group: CollisionGroups,
    pub marker: TerrainMarker,
    //pub collider: Collider,
}

impl TerrainBundle {
    pub fn new(
        ga: &Res<GameAssets>,
        gltf_assets: &Res<Assets<Gltf>>,
        
    ) -> Self {
        let terrain_scene = TerrainSceneHandle::new(ga, gltf_assets);

        Self {
            terrain_scene: SceneBundle {
                scene: terrain_scene.terrain_scene.clone(),
                ..Default::default()
            },
            //collider: Collider::default(), // Capsule collider with height and radius
            collision_group: CollisionGroups::new(
                Group::from_bits_truncate(ENVIRONMENT_GROUP),
                Group::from_bits_truncate(PLAYER_GROUP)
            ),
            marker: TerrainMarker,
        }
    }
}
        

 