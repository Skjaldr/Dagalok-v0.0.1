use bevy::prelude::*;
use leafwing_input_manager::{action_state::ActionState, Actionlike};

use crate::{module_bindings::{PlayerAction, PlayerStances}, player::player_bundle::{Player, PlayerEntity}};

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GameActions {
    Forward,
    Back,
    Left,
    Right,
    Jump,
    Crouch,
    CombatStance,
    Equip,
    Attack,
}

pub fn get_input_vector(
    action_state: &ActionState<GameActions>,
    cam_q: &Query<&Transform, (With<Camera3d>, Without<Player>)>,
) -> Vec3 {
    
    let cam = match cam_q.get_single() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error retrieving camera: {}", e);
            return Vec3::ZERO;
        }
    };

    let mut direction = Vec3::ZERO;

    if action_state.pressed(&GameActions::Forward) && action_state.pressed(&GameActions::Right) {
        direction += *cam.forward() + *cam.right();
    } else if action_state.pressed(&GameActions::Forward) && action_state.pressed(&GameActions::Left) {
        direction += *cam.forward() + *cam.left();
    } else if action_state.pressed(&GameActions::Back) && action_state.pressed(&GameActions::Left) {
        direction += *cam.back() + *cam.left();
    } else if action_state.pressed(&GameActions::Back) && action_state.pressed(&GameActions::Right) {
        direction += *cam.back() + *cam.right();
    } else {
        match action_state {
            state if state.pressed(&GameActions::Forward) => {
                println!("Moving Forward");
                direction += *cam.forward();
            },
            state if state.pressed(&GameActions::Back) => {
                println!("Moving Back");
                direction += *cam.back();
            },
            state if state.pressed(&GameActions::Left) => {
                println!("Moving Left");
                direction += *cam.left();
            },
            state if state.pressed(&GameActions::Right) => {
                println!("Moving Right");
                direction += *cam.right();
            },
            state if state.just_pressed(&GameActions::Jump) => {
                println!("Jumping");
                direction.y += 5.5;
            }
            _ => {}
        }
    }

    Vec3 {
        x: direction.x,
        y: direction.y,
        z: direction.z,
    }
}

pub fn handle_stance_change(
    //mut player_q: &mut Query<(&ActionState<GameActions>, &mut PlayerEntity)>,
    action_state: &ActionState<GameActions>,
    entity: &mut PlayerEntity
) -> PlayerStances {
    //for (action_state, mut entity) in player_q.iter_mut() {
        if action_state.just_pressed(&GameActions::CombatStance) {
            // Update player stance
                entity.data.stance = match entity.data.stance {
                    PlayerStances::NonCombat => PlayerStances::Combat,
                    PlayerStances::Combat => PlayerStances::NonCombat,
                    _ => entity.data.stance.clone(),
                };
                println!("Player stance changed to: {:?}", entity.data.stance);
                
                return entity.data.stance.clone();
        } else {
            return entity.data.stance.clone();
        }
    
}

pub fn handle_attack(
    //mut player_q: &mut Query<(&ActionState<GameActions>, &mut PlayerEntity)>,
    action_state: &ActionState<GameActions>,
    entity: &mut PlayerEntity
) -> PlayerAction {
    //for (action_state, mut entity) in player_q.iter_mut() {
        if action_state.just_pressed(&GameActions::Attack) {
            // Update player stance
               entity.data.action = match entity.data.action {
                PlayerAction::None =>
                PlayerAction::Attack,

                _ => entity.data.action.clone()
               };

               println!("Player Attacked {:?}", entity.data.action);
               return entity.data.action.clone();
        } else {
            return entity.data.action.clone();
        }
    
}