pub mod run_animations;
pub mod setup_animations;

use bevy::{animation::animate_targets, prelude::*};
use run_animations::{link_animations, play_animation};
use setup_animations::{get_animations, AnimationController, Animations};
use crate::{asset_manager::AssetLoadingState, player::HandleScenesState};


pub struct AnimPlugin;
impl Plugin for AnimPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Animations::default())
        .add_systems(OnEnter(AssetLoadingState::LoadingAnimations), get_animations)
        //.add_systems(OnEnter(AnimationLoadingState::LoadingSetup))
        //.add_systems(OnEnter(HandleScenesState::Spawned), animation_list)
        .add_systems(Update, (link_animations, play_animation.before(animate_targets)).run_if(in_state(HandleScenesState::Done)));
    }
}