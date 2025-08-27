mod network;
mod player;
mod terrain;
pub mod input;
mod module_bindings;
mod asset_manager;
mod camera;
mod animations;

use animations::AnimPlugin;
use asset_manager::LoadAssetPlugin;
use bevy_rapier3d::{plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use camera::CameraPlugin;
use bevy::prelude::*;
use bevy_third_person_camera::*;
use futures_channel::mpsc;
use input::GameActions;
use leafwing_input_manager::plugin::InputManagerPlugin;
use network::{connect_and_register::{connect_to_db, register_callbacks, subscribe_to_tables}, uncb_receiver, NetworkPlugin};
use player::{HandleScenesPlugin, HandleScenesState};
use terrain::TerrainPlugin;
use uncb_receiver::{
    process_messages, 
    UncbEvent, UncbMessage, UncbReceiver, UncbSend,
};
//use player_world::TerrainPlugin;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    LoadingTerrain,
    SpawningPlayer,
    InGame,
}

impl States for GameState {}


fn main() {
    
    // Initialize your logger
    env_logger::init();

    let (uncb_send, uncb_recv) = mpsc::unbounded();

    register_callbacks(uncb_send.clone());
    connect_to_db();
    subscribe_to_tables();

    let mut app = App::new();
    app.insert_resource(UncbReceiver::new(uncb_recv))
        .add_event::<UncbEvent>()
        .add_plugins((
            NetworkPlugin,
            DefaultPlugins,
            CameraPlugin,
            LoadAssetPlugin,
            HandleScenesPlugin,
            AnimPlugin,
            ThirdPersonCameraPlugin,
            TerrainPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            InputManagerPlugin::<GameActions>::default(),
            //RapierDebugRenderPlugin::default(),
        ))
        //.init_state::<GameState>()
        .add_systems(OnEnter(HandleScenesState::Spawned), setup)
        .run();
}

fn setup(
    mut rapier_config: ResMut<RapierConfiguration>,
    mut next_state: ResMut<NextState<HandleScenesState>>
) {
    rapier_config.gravity = Vec3::new(0.0, -9.81, 0.0);
    println!("Gravity set to {:?}", rapier_config.gravity); // Debug print

    next_state.set(HandleScenesState::Done);
}