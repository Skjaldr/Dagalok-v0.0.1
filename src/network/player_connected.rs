use bevy::prelude::*;
use crate::player::player_bundle::Player;

use super::uncb_receiver::{UncbEvent, UncbMessage};


#[derive(Resource, Default)]
pub struct PlayerDataResource {
    pub player_vec: Vec<Player>
}

// Component tag for players who are not the main player

pub fn player_connected_data(
    mut commands: Commands,
    mut player_data: ResMut<PlayerDataResource>,
    mut event_reader: EventReader<UncbEvent>,

) {
    let mut new_non_main_player_inserted = false;
    commands.insert_resource(NewPlayer(new_non_main_player_inserted));
    //let mut player_map = Vec::new();

    for event in event_reader.read() {
        if let UncbMessage::PlayerInserted { data, event: _ } = &event.message {
            println!("{:?}", data);
            info!("PlayerInserted event received for player: {:?}", data.owner_id);

            if !player_data.player_vec.iter().any(|p| p.data.entity_id == data.entity_id) {
                
                if data.owner_id != spacetimedb_sdk::identity::identity().unwrap() {
                    new_non_main_player_inserted = true;
                    
                } 

                player_data.player_vec.push(Player{data: data.clone()});
                
                commands.insert_resource(
                    PlayerDataResource {
                        player_vec: player_data.player_vec.clone()
                    }
                );
            }
        }
    }

    if new_non_main_player_inserted {
        println!("NEW NON-MAIN PLAYER ENTERS THE FREY");

        let np = NewPlayer(new_non_main_player_inserted);
        commands.insert_resource(np);

    }
}

#[derive(Resource, Default)]
pub struct NewPlayer(pub bool);