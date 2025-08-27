mod terrain_setup;
mod terrain_bundle;
use bevy::prelude::*;
use terrain_bundle::TerrainSceneHandle;
use terrain_setup::setup_terrain;
//use terrain_setup::spawn_terrain;

use crate::asset_manager::AssetLoadingState;

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TerrainSceneHandle>()
            .add_systems(OnExit(AssetLoadingState::LoadingAnimations), setup_terrain);  
    }
}

