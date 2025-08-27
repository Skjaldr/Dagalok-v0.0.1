use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{input::{get_input_vector, handle_attack, handle_stance_change, GameActions}, module_bindings::{entity_component, update_player_action, update_player_position, update_player_stance, StdbVector3}, network::{uncb_receiver::{UncbEvent, UncbMessage}, vec3_nan_to_zero}};

use super::{player_bundle::{Player, PlayerEntity}, spawn_player::PlayerEntities};

pub fn player_movement(
    mut player_q: ParamSet<
        (
            Query<(
                Option<&ActionState<GameActions>>,
                &mut Transform,
                &mut PlayerEntity,
                Option<&mut InterpolatedTransform>
        ), With<Player>>,
    )>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    time: Res<Time>
) {
    for (action_state, mut transform, mut player_entity, interpolated_transform) in 
    player_q.p0().iter_mut() {

        let player_speed = 2.0;
        let mut is_moving = false;
        let mut new_direction = player_entity.data.direction;

        if let Some(action_state) = action_state {
    
            let input_vector = vec3_nan_to_zero(
                get_input_vector(
                    action_state, 
                    &cam_q,
                    //&mut entity_query,
                ).normalize_or_zero()) * time.delta_seconds();
                
            let mut horizontal_movement = input_vector * player_speed;
            let _vertical_movement = if action_state.pressed(&GameActions::Jump) {
                player_speed * time.delta_seconds()
            } else {
                0.0
            };

            if horizontal_movement.length_squared() > 0.0 {
                is_moving = true;
                horizontal_movement.y = 0.0;
                
                // Correcting direction calculation using atan2
                new_direction = f32::atan2(horizontal_movement.x * -1.0, horizontal_movement.z * -1.0);

                transform.look_to(horizontal_movement.normalize(), Vec3::Y);
            }

            transform.translation.x += horizontal_movement.x;
            transform.translation.z += horizontal_movement.z;

            // Jump - Movement along the Y axis requires special attention.
            if action_state.pressed(&GameActions::Jump) {
                transform.translation.y += player_speed * time.delta_seconds();
            }

            let new_stance = handle_stance_change(action_state, &mut player_entity);
            let new_action = handle_attack(action_state, &mut player_entity);

            // Sync to the database.
            update_player_position(StdbVector3 {
                x: transform.translation.x,
                y: transform.translation.y,
                z: transform.translation.z,
            },
                new_direction,
                is_moving,
            );
            update_player_stance(new_stance.clone());
            update_player_action(new_action.clone());
            // Update player entity direction and stance
            player_entity.data.stance = new_stance;
            player_entity.data.direction = new_direction;
        } else {
            // Read from the database and update transform.
            if let Some(entity) = entity_component::EntityComponent::filter_by_entity_id(player_entity.data.entity_id).next() {
                let position = entity.position;
                let direction = entity.direction;
                let stance = entity.stance;
                let action = entity.action;

                if let Some(mut interpolated_transform) = interpolated_transform {
                    interpolated_transform.target_position = Vec3::new(position.x, position.y, position.z);
                    interpolated_transform.progress = 0.0;
                } else {
                    transform.translation = Vec3::new(position.x, position.y, position.z);
                }

                // Apply the stored direction
                transform.rotation = Quat::from_rotation_y(direction);

                // Ensure player entity is updated from the database
                player_entity.data.position = position.clone();
                player_entity.data.direction = direction.clone();
                player_entity.data.stance = stance.clone();
                player_entity.data.action = action.clone();
                //println!("Player entity's stance: {:?}", player_entity.data.stance);
            } else {
                warn!("EntityComponent not found for entity_id: {:?}", player_entity.data.entity_id);
            }
        }
    }
}



pub fn interpolate_positions(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut InterpolatedTransform)>,
) {
    for (mut transform, mut interpolated_transform) in query.iter_mut() {
        let progress = (interpolated_transform.progress + time.delta_seconds()).min(1.0);
        transform.translation = transform
            .translation
            .lerp(interpolated_transform.target_position, progress);
        interpolated_transform.progress = progress;
    }
}

#[derive(Component)]
pub struct InterpolatedTransform {
    pub target_position: Vec3,
    pub progress: f32,
}

impl Default for InterpolatedTransform {
    fn default() -> Self {
        Self {
            target_position: Vec3::ZERO,
            progress: 1.0,
        }
    }
}