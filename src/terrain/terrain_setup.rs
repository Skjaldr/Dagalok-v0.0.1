use bevy::{prelude::*, render::mesh::Indices, utils::HashMap};
use bevy_rapier3d::prelude::Collider;
use crate::{asset_manager::{AssetLoadingState, GameAssets}, player::HandleScenesState};

use super::{terrain_bundle::TerrainBundle};

#[derive(Resource, Debug)]
pub struct TerrainEntititiesByName(pub HashMap<String, Entity>);


pub fn setup_terrain(
    mut commands: Commands,
    mesh: Res<Assets<Mesh>>,
    ga: Res<GameAssets>,
    gltf_assets: Res<Assets<Gltf>>,
    scene: Res<Assets<Scene>>,
    mut next_state: ResMut<NextState<AssetLoadingState>>,
    mut spawning_state: ResMut<NextState<HandleScenesState>>
) {

    println!("Loading Terrain");
    let scene_entities_by_name: HashMap<String, Entity> = HashMap::new();
    let collider = terrain_collider(
        &mesh, 
        &ga, 
        &gltf_assets, 
        &scene
    );
    let terrain = TerrainBundle::new(&ga, &gltf_assets);

    commands.spawn(terrain).insert(collider);
    commands.insert_resource(TerrainEntititiesByName(scene_entities_by_name));

    println!("Terrain Spawned");
    next_state.set(AssetLoadingState::Spawning);
    spawning_state.set(HandleScenesState::Spawning);
}

pub fn terrain_collider(

    mesh: &Res<Assets<Mesh>>,
    ga: &Res<GameAssets>,
    gltf_assets: &Res<Assets<Gltf>>,
    scene: &Res<Assets<Scene>>,

) -> Collider {
    let mut collider = Collider::default();
   for (_, gltf_handle) in &ga.terrain_files {
        if let Some(gltf) = gltf_assets.get(gltf_handle) {
            let earth = gltf.named_scenes["Scene"].clone();
            if let Some(earth_scene) = &scene.get(&earth) {
                let world = &earth_scene.world;
                let mut p_vec: Vec<Vec3> = vec![];
                let mut i_vec: Vec<[u32; 3]> = vec![];

                for entity_ref in world.iter_entities() {
                    if let Some(mesh_handle) = world.get::<Handle<Mesh>>(entity_ref.id()) {
                        if let Some(meshes) = mesh.get(mesh_handle) {
                            let positions = match meshes.attribute(Mesh::ATTRIBUTE_POSITION) {
                                Some(attr) => match attr.as_float3() {
                                    Some(pos) => pos,
                                    None => {
                                        println!("Error: ATTRIBUTE_POSITION is not a float3");
                                        return Collider::default();
                                    }
                                },
                                None => {
                                    println!("Error: Mesh has no ATTRIBUTE_POSITION");
                                    return Collider::default();
                                }
                            };

                            let indices = match meshes.indices() {
                                Some(Indices::U32(indices)) => indices.clone(),
                                Some(Indices::U16(indices)) => indices.iter().map(|&i| i as u32).collect::<Vec<_>>(),
                                None => {
                                    println!("Error: Mesh does not contain indices");
                                    return Collider::default();
                                }
                            };

                            let offset = p_vec.len() as u32;
                            p_vec.extend(positions.iter().map(|&v| Vec3::new(v[0], v[1], v[2])));
                            i_vec.extend(indices.chunks(3).map(|chunk| [chunk[0] + offset, chunk[1] + offset, chunk[2] + offset]));

                            
                        }
                    }
                }

                if !p_vec.is_empty() && !i_vec.is_empty() {
                    collider = Collider::trimesh(p_vec.clone(), i_vec.clone());
                }
            }
        }
    }
    
    return collider;
}