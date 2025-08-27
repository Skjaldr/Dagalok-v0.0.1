use bevy::prelude::*;

use crate::{animations::setup_animations::Animations, player::player_bundle::PlayerEntity};

use super::{player_connected::PlayerDataResource, uncb_receiver::{UncbEvent, UncbMessage}};

pub fn handle_player_updates(
    mut event_reader: EventReader<UncbEvent>,
    mut player_data: ResMut<PlayerDataResource>,
    query: Query<&PlayerEntity>,
    animations: Res<Animations>,
) {

    for event in event_reader.read() {
        if let UncbMessage::EntityUpdated { old, new, event: _, } = &event.message {
            for players in player_data.player_vec.iter() {
                
                
            }
        }
    }
}