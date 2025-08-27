use bevy::{prelude::*, utils::HashMap};
use bevy::gltf::Gltf;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};


//use animations::run_animations::AnimationLoadingState;


pub struct LoadAssetPlugin;
impl Plugin for LoadAssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AssetLoadingState>()
            .add_loading_state(LoadingState::new(AssetLoadingState::Loading)
            .continue_to_state(AssetLoadingState::LoadingAnimations)
            .load_collection::<GameAssets>());
        

    }
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AssetLoadingState {
    #[default]
    Loading,
    LoadingAnimations,
    Spawning,
    _Done,
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(paths("models\\target_3.glb"), collection(typed, mapped))]
    pub gltf_files: HashMap<String, Handle<Gltf>>,

    #[asset(paths("models\\source_4.glb"), collection(typed, mapped))]
    pub source: HashMap<String, Handle<Gltf>>,

    #[asset(paths("earth_floor.glb"), collection(typed, mapped))]
    pub terrain_files: HashMap<String, Handle<Gltf>>,

    #[asset(paths("models\\axe.glb"), collection(typed, mapped))]
    pub _weapons: HashMap<String, Handle<Gltf>>,
    
}
