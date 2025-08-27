pub(crate) mod uncb_receiver;
pub(crate) mod player_connected;
pub(crate) mod connect_and_register;
pub(crate) mod player_updates;

use bevy::prelude::*;


use player_connected::{player_connected_data, NewPlayer, PlayerDataResource};
use uncb_receiver::process_messages;

use crate::module_bindings::create_player;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_resource::<PlayerDataResource>()
            .init_resource::<NewPlayer>()
            .add_systems(Startup, create_player)
            .add_systems(Update, (process_messages, player_connected_data));
    }
}


//#region helpers
pub fn nan_to_zero(v: f32) -> f32 {
    if v.is_nan() {
        0.0
    } else {
        v
    }
}

pub fn vec3_nan_to_zero(v: Vec3) -> Vec3 {
    Vec3 {
        x: nan_to_zero(v.x),
        y: nan_to_zero(v.y),
        z: nan_to_zero(v.z),
    }
}
//#endregion helpers
