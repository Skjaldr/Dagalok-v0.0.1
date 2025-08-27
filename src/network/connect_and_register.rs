//use bevy::prelude::*;
use log::{error, info};
use super::uncb_receiver;
use uncb_receiver::{UncbMessage, UncbSend};

use spacetimedb_sdk::{
    Address,
    //disconnect,
    identity::{load_credentials, once_on_connect, save_credentials, Credentials, Identity},
    on_disconnect, //on_subscription_applied,
    subscribe,
    table::{TableType, TableWithPrimaryKey},
};

use crate::{module_bindings::{client, connect, Client, EntityComponent, PlayerComponent, ReducerEvent}, 
//GameState
};



const SPACETIMEDB_URI: &str = "http://127.0.0.1:3000";
const DB_NAME: &str = "dagalok";
const CREDS_DIR: &str = ".dagalok";
const DEBUG_MODE: bool = true;

/// Register all the callbacks our app will use to respond to database events.
pub fn connect_to_db() {
    connect(
        SPACETIMEDB_URI,
        DB_NAME,
        if DEBUG_MODE {
            None
        } else {
            load_credentials(CREDS_DIR).expect("Error reading stored credentials")
        },
    )
    .expect("Failed to connect");
}


/// Register subscriptions for all rows of tables.
pub fn subscribe_to_tables() {
    if let Err(e) = subscribe(&["SELECT * FROM *"]) {
        error!("Failed to subscribe to tables: {:?}", e);
    }
}


//#region callbacks
pub fn register_callbacks(uncb_send: UncbSend) {
    once_on_connect(on_connected(uncb_send.clone()));
    on_disconnect(on_disconnected(uncb_send.clone()));

    EntityComponent::on_insert(on_entity_inserted(uncb_send.clone()));
    EntityComponent::on_update(on_entity_updated(uncb_send.clone()));
    EntityComponent::on_delete(on_entity_deleted(uncb_send.clone()));

    client::Client::on_insert(on_client_inserted(uncb_send.clone()));
    client::Client::on_update(on_client_updated(uncb_send.clone()));

    PlayerComponent::on_insert(on_player_inserted(uncb_send.clone()));
    PlayerComponent::on_update(on_player_updated(uncb_send.clone()));
    PlayerComponent::on_delete(on_player_deleted(uncb_send.clone()));
}

fn on_connected(uncb_send: UncbSend) -> impl FnMut(&Credentials, Address) + Send + 'static {
    move |creds, address| {
        if let Err(e) = save_credentials(CREDS_DIR, creds) {
            eprintln!("Failed to save credentials: {:?}", e);
        }
        uncb_send
            .unbounded_send(UncbMessage::Connected {
                creds: creds.clone(),
                address,
            })
            .unwrap();

        // Fetch existing players and synchronize state
        for player_component in PlayerComponent::iter() {
            uncb_send
                .unbounded_send(UncbMessage::PlayerInserted {
                    data: player_component.clone(),
                    event: None,
                })
                .unwrap();
        }
    }
}



fn on_disconnected(uncb_send: UncbSend) -> impl FnMut() + Send + 'static {
    move || {
        eprintln!("Disconnected!");
        uncb_send.unbounded_send(UncbMessage::Disconnected).unwrap();
        std::process::exit(0)
    }
}

fn on_entity_inserted(
    uncb_send: UncbSend,
) -> impl FnMut(&EntityComponent, Option<&ReducerEvent>) + Send + 'static {
    move |entity, event| {
        if let Some(event) = event {
            uncb_send
                .unbounded_send(UncbMessage::EntityInserted {
                    data: entity.clone(),
                    event: Some(event.clone()),
                })
                .unwrap();
        }
    }
}

fn on_entity_updated(
    uncb_send: UncbSend,
) -> impl FnMut(&EntityComponent, &EntityComponent, Option<&ReducerEvent>) + Send + 'static {
    move |old, new, event| {
        if let Some(event) = event {
            uncb_send
                .unbounded_send(UncbMessage::EntityUpdated {
                    new: new.clone(),
                    old: old.clone(),
                    event: Some(event.clone()),
                })
                .unwrap();
        }
    }
}

fn on_entity_deleted(
    uncb_send: UncbSend,
) -> impl FnMut(&EntityComponent, Option<&ReducerEvent>) + Send + 'static {
    move |entity, event| {
        if let Some(event) = event {
            uncb_send
                .unbounded_send(UncbMessage::EntityRemoved {
                    data: entity.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn on_client_inserted(
    _uncb_send: UncbSend,
) -> impl FnMut(&Client, Option<&ReducerEvent>) + Send + 'static {
    move |client, _event| {
        if client.connected {
            println!(
                "Client {} connected.",
                identity_leading_hex(&client.client_id)
            );
        }
    }
}

fn on_client_updated(
    mut _uncb_send: UncbSend,
) -> impl FnMut(&Client, &Client, Option<&ReducerEvent>) + Send + 'static {
    move |old, new, _event| {
        if old.connected && !new.connected {
            println!(
                "Client {} disconnected.",
                identity_leading_hex(&new.client_id)
            );
        }
        if !old.connected && new.connected {
            println!("Client {} connected.", identity_leading_hex(&new.client_id));
        }
    }
}

fn on_player_inserted(
    uncb_send: UncbSend,
) -> impl FnMut(&PlayerComponent, Option<&ReducerEvent>) + Send + 'static {
    move |player, event| {
        info!("Player component inserted: {:?}", player);
        uncb_send
            .unbounded_send(UncbMessage::PlayerInserted {
                data: player.clone(),
                event: event.cloned(),
            })
            .unwrap();
    }
}

fn on_player_updated(
    uncb_send: UncbSend,
) -> impl FnMut(&PlayerComponent, &PlayerComponent, Option<&ReducerEvent>) + Send + 'static {
    move |old, new, event| {
        if let Some(event) = event {
            info!("UncbMessage::PlayerUpdated called");
            uncb_send
                .unbounded_send(UncbMessage::PlayerUpdated {
                    old: old.clone(),
                    new: new.clone(),
                    event: Some(event.clone()),
                })
                .unwrap();
        }
    }
}

fn on_player_deleted(
    uncb_send: UncbSend,
) -> impl FnMut(&PlayerComponent, Option<&ReducerEvent>) + Send + 'static {
    move |player, _event| {
        info!("UncbMessage::PlayerRemoved called");
        uncb_send
            .unbounded_send(UncbMessage::PlayerRemoved {
                data: player.clone(),
            })
            .unwrap();
    }
}

fn identity_leading_hex(id: &Identity) -> String {
    hex::encode(&id.bytes()[0..8])
}
//#endregion callbacks



// fn log_transitions(mut transitions: EventReader<StateTransitionEvent<GameState>>) {
//     for transition in transitions.read() {
//         info!(
//             "transition: {:?} => {:?}",
//             transition.entered, transition.exited
//         );
//     }
// }

// end of main.rs