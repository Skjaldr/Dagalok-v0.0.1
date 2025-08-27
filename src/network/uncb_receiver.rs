//uncb_receiver.rs
use bevy::prelude::*;
use futures_channel::mpsc;
use log::info;
use spacetimedb_sdk::{identity::Credentials, Address};

use crate::module_bindings::{EntityComponent, PlayerComponent, ReducerEvent};


/// Unbound Callback Message
/// Used to tell our unbounded reciever what \
/// specific event has occured while passing params.
/// [System based on this](https://github.com/clockworklabs/SpacetimeDB/blob/master/crates/sdk/examples/cursive-chat/main.rs#L45)


#[derive(Clone, Debug, Event)]
pub enum UncbMessage {
    Connected {
        creds: Credentials,
        address: Address,
    },
    Disconnected,
    PlayerInserted {
        data: PlayerComponent,
        event: Option<ReducerEvent>,
    },
    PlayerUpdated {
        old: PlayerComponent,
        new: PlayerComponent,
        event: Option<ReducerEvent>,
    },
    PlayerRemoved {
        data: PlayerComponent,
    },
    EntityInserted {
        data: EntityComponent,
        event: Option<ReducerEvent>,
    },
    EntityUpdated {
        old: EntityComponent,
        new: EntityComponent,
        event: Option<ReducerEvent>,
    },
    EntityRemoved {
        data: EntityComponent,
        event: ReducerEvent,
    },
}


pub type UncbSend = mpsc::UnboundedSender<UncbMessage>;
pub type UncbRecv = mpsc::UnboundedReceiver<UncbMessage>;

#[derive(Resource)]
pub struct UncbReceiver {
    pub recv: UncbRecv,
}

impl UncbReceiver {
    pub fn new(recv: UncbRecv) -> Self {
        UncbReceiver { recv }
    }
}

#[derive(Event, Debug)]
pub struct UncbEvent {
    pub message: UncbMessage,
}


pub fn process_messages(
    mut res: ResMut<UncbReceiver>, 
    mut commands: Commands
) {
    let mut messages = vec![];
    
    loop {
        let message = res.recv.try_next();
        if let Ok(message) = message {
            if let Some(message) = message {
                messages.push(message);
            }
        } else {
            break;
        }
    }

    for message in messages {
        commands.add(move |world: &mut World| {
            world.send_event(UncbEvent { message: message.clone() });
        });
    }
}