use spacetimedb::{spacetimedb, Identity, SpacetimeType, ReducerContext};
use log::info;


// We're using this table as a singleton, so there should typically only be one element where the version is 0.
// #[spacetimedb(table)]
// #[derive(Clone)]
// pub struct Config {
//     #[primarykey]
//     pub version: u32,
//     pub message_of_the_day: String,
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq, SpacetimeType, Default)]
pub enum PlayerStances {
    #[default]
    NonCombat,
    Combat,
    Precise,
    Defensive,
}

#[derive(Clone, Debug, PartialEq, Eq, SpacetimeType, Default)]
pub enum PlayerAction {
    #[default]
    None,
    Attack,
    Block,
    Dodge,
    UseItem(u32),
    CastSpell(String),
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct Client {
    #[primarykey]
    pub client_id: Identity,
    pub connected: bool,
}

// This allows us to store 3D points in tables.
#[derive(SpacetimeType, Clone, Default)]
pub struct StdbVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// This stores information related to all entities in our game. In this tutorial
// all entities must at least have an entity_id, a position, a direction and they
// must specify whether or not they are moving.
#[spacetimedb(table(public))]
#[derive(Clone, Default)]
pub struct EntityComponent {
    #[primarykey]
    // The autoinc macro here just means every time we insert into this table
    // we will receive a new row where this value will be increased by one. This
    // allows us to easily get rows where `entity_id` is unique.
    #[autoinc]
    pub entity_id: u64,
    pub position: StdbVector3,
    pub direction: f32,
    pub moving: bool,
    pub stance: PlayerStances,
    pub action: PlayerAction,
}

// All players have this component and it associates an entity with the user's
// Identity. It also stores their username and whether or not they're logged in.
#[derive(Clone)]
#[spacetimedb(table(public))]
pub struct PlayerComponent {
    // An entity_id that matches an entity_id in the `EntityComponent` table.
    #[primarykey]
    pub entity_id: u64,

    // The user's identity, which is unique to each player
    #[unique]
    pub owner_id: Identity,
    //pub username: String,
    //pub logged_in: bool,
}

#[spacetimedb(init)]
pub fn init() {
   
}

// This reducer is called when the user logs in for the first time and
// enters a username
#[spacetimedb(reducer)]
pub fn create_player(ctx: ReducerContext) -> Result<(), String> {
    // Get the Identity of the client who called this reducer
    let owner_id = ctx.sender;

    // make sure a player doesn't already exist with this identity

    if PlayerComponent::filter_by_owner_id(&owner_id).is_some() {
        log::info!("Player already exists");
        return Err("Player already exists".to_string());
    }

    let entity_id = EntityComponent::insert(EntityComponent::default())
        .expect("Failed to create a unique ObComponent.")
        .entity_id;

    PlayerComponent::insert(PlayerComponent {
        entity_id,
        owner_id,
    })
    .expect("Failed to insert player component.");

    log::info!("Player created: {})", entity_id);

    Ok(())
}


// Called when the client connects, we update the logged_in state to true
#[spacetimedb(connect)]
pub fn client_connected(ctx: ReducerContext) {
    update_player_login_state(ctx, true);
}

// Called when the client disconnects, we update the logged_in state to false
#[spacetimedb(disconnect)]
pub fn client_disconnected(ctx: ReducerContext) {
    update_player_login_state(ctx, false);
}

// This helper function gets the PlayerComponent, sets the logged
// in variable and updates the PlayerComponent table row.
pub fn update_player_login_state(ctx: ReducerContext, connected: bool) {
    if let Some(client) = Client::filter_by_client_id(&ctx.sender) {
            let mut client = client.clone();
            client.connected = connected;
            Client::update_by_client_id(&ctx.sender, client);

                if !connected {
                    remove_player(ctx).expect("player doesn't exist");
                }
                info!("Updated client Login State");
        } else {
            Client::insert(Client {client_id: ctx.sender,connected})
            .expect("Failed to create unique Client");
            info!("Created Client");
        }
}

pub fn remove_player(ctx: ReducerContext) -> Result<(), String> {
    
    let client_id = ctx.sender;

    if !PlayerComponent::filter_by_owner_id(&client_id).is_some() {
        log::info!("Player doesn't exist");
        return Err("Player doesn't exist".to_string());
    }

    if let Some(player) = PlayerComponent::filter_by_owner_id(&ctx.sender) {
        let _player = player.clone();
        PlayerComponent::delete_by_owner_id(&client_id);
        log::info!("Removed Player: {}", _player.owner_id);
    }

    Ok(())
}

// Updates the position of a player. This is also called when the player stops moving.
#[spacetimedb(reducer)]
pub fn update_player_position(
    ctx: ReducerContext,
    position: StdbVector3,
    direction: f32,
    moving: bool,
    //stance: PlayerStances,
) -> Result<(), String> {
    if let Some(player) = PlayerComponent::filter_by_owner_id(&ctx.sender) {
        if let Some(mut entity) = EntityComponent::filter_by_entity_id(&player.entity_id) {
            entity.position = position;
            entity.direction = direction;
            entity.moving = moving;
            //entity.stance = stance; // Update the stance
            EntityComponent::update_by_entity_id(&player.entity_id, entity);
            return Ok(());
        }
    }

    Err("Player not found".to_string())
}

#[spacetimedb(reducer)]
pub fn update_player_action(
    ctx: ReducerContext,
    action: PlayerAction,
) -> Result<(), String> {
    if let Some(player) = PlayerComponent::filter_by_owner_id(&ctx.sender) {
        if let Some(mut entity) = EntityComponent::filter_by_entity_id(&player.entity_id) {
            entity.action = action;
            EntityComponent::update_by_entity_id(&player.entity_id, entity);
            return Ok(());
        }
    }
    return Err("Player not found".to_string())
}

#[spacetimedb(reducer)]
pub fn update_player_stance(
    ctx: ReducerContext,
    stance: PlayerStances,
) -> Result<(), String> {
    if let Some(player) = PlayerComponent::filter_by_owner_id(&ctx.sender) {
        if let Some(mut entity) = EntityComponent::filter_by_entity_id(&player.entity_id) {
            entity.stance = stance;
            EntityComponent::update_by_entity_id(&player.entity_id, entity);
            return Ok(());
        }
    }
    return Err("Player not found".to_string())
}