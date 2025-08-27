use std::{ops::Index, time::Duration};

use bevy::{animation::ActiveAnimation, input::keyboard::{Key, KeyboardInput}, prelude::*, utils::HashMap};
use crate::{asset_manager, input::GameActions, module_bindings::{entity_component, PlayerAction, PlayerStances}, network::uncb_receiver::{UncbEvent, UncbMessage}, player::{player_bundle::{Player, PlayerEntity}, spawn_player::NonMainPlayer}};
use super::setup_animations::{AnimationController, GraphAnimations};


pub fn play_animation(
    mut commands: Commands,
    mut animation_player_query: Query<&mut AnimationPlayer>,
    mut player_character_query: Query<(&Player, &AnimationEntityLink, &PlayerEntity, &AnimationController)>,
    // list: Res<AnimationList>,
    graph: Res<GraphAnimations>,
    mut event_reader: EventReader<UncbEvent>,
    name: Query<&Name>,
) {
    
    //let transition = AnimationTransitions::new();
    //let anim_list = Animations::new(&list);
    for event in event_reader.read() {
        if let UncbMessage::EntityUpdated { new, .. } = &event.message {
            for (player, animation_entity_link, player_entity, controller) in player_character_query.iter_mut() {

                if player.data.entity_id == new.entity_id {

                    let idle_sword = controller.animations.index_node.get("Idle_Sword_And_Shield").expect("No").clone();
                    let run_sword = controller.animations.index_node.get("Run_Sword_And_Shield").expect("No").clone();


                    let run = controller.animations.index_node.get("Run_Standard").expect("No").clone();

                    let idle = controller.animations.index_node.get("Idle").expect("No").clone();

                    let attack = controller.animations.index_node.get("Attack_Sword_And_Shield_Slash").expect("No").clone();

                    if let Ok(mut animation_player) = animation_player_query.get_mut(animation_entity_link.0) {
                        let current = match player_entity.data.stance {
                            PlayerStances::Combat => {
                               
                                if new.moving {
                                   
                                    run_sword
                                } else {
                                    idle_sword
                                }
                            }
                            PlayerStances::NonCombat => {
                                if new.moving {
                                    run
                                } else {
                                    idle
                                }
                            }
                            _ => {
                                if !new.moving {
                                    idle
                                } else {
                                    run
                                }
                                
                            }
                        };
                        

                       let action =  match player_entity.data.action {
                            PlayerAction::Attack => {
                               
                                attack
                            },
                            _=> current
                        };

                       
                        if !animation_player.is_playing_animation(current) {
                            
                            animation_player.stop_all();
                            animation_player.play(current.clone()).repeat();
                            //animation_player.play(action);
                            if action != current {
                                animation_player.stop_all();
                                animation_player.play(action);
                            }

                            
                            

                       }
                    }
                }
                commands.entity(animation_entity_link.0).insert(graph.graph.clone());
            }
        }
    }
}

#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);

fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

pub fn link_animations(
    player_query: Query<Entity, With<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);
        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            //warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}
