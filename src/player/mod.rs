pub mod player_bundle;
pub mod spawn_player;
mod movement;
//mod player_updates;

use movement::{interpolate_positions, player_movement};
use player_bundle::{Player, PlayerEntity};
use bevy::prelude::*;
use spawn_player::{spawn_new_players, spawn_players};
use super::module_bindings;

use crate::{asset_manager::AssetLoadingState, module_bindings::{create_player, Client}, GameState};

#[derive(States, Clone, Eq, PartialEq, Hash, Default, Debug)]
pub enum HandleScenesState {
    #[default]
    Spawning,
    Spawned,
    Done,  
}

pub struct HandleScenesPlugin;
impl Plugin for HandleScenesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<HandleScenesState>()
            .add_systems(Startup, create_player)
            .add_systems(OnEnter(AssetLoadingState::Spawning), spawn_players)
            .add_systems(Update, log_transitions)
            .add_systems(Update, (player_movement, interpolate_positions).run_if(in_state(HandleScenesState::Done)))
            .add_systems(Update, spawn_new_players.run_if(in_state(HandleScenesState::Done)));

            
    }
}

fn log_transitions(mut transitions: EventReader<StateTransitionEvent<HandleScenesState>>) {


    for transition in transitions.read() {
        info!(
            "transition: {:?} => {:?}",
            transition.exited, transition.entered
        );
    }
}