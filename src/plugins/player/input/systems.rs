use bevy::prelude::{Query, With};
use leafwing_input_manager::action_state::ActionState;
use crate::input::actions::Actions;
use crate::plugins::player::components::PlayerEntity;
use crate::plugins::player::input::components::PlayerInputState;

pub(crate) fn player_input(
    mut query_player: Query<
        (
            &mut PlayerInputState
        ),
        With<PlayerEntity>
    >,
    query_action_state: Query<&ActionState<Actions>>
) {
    let action_state = query_action_state.single();

    let action_move = action_state.axis_pair(&Actions::PlayerMove).unwrap().xy();
    let action_point = action_state.axis_pair(&Actions::PlayerLook).unwrap().xy().normalize_or_zero();

    let action_jump = action_state.pressed(&Actions::PlayerJump);
    let action_sprint = action_state.pressed(&Actions::PlayerSprint);
    let action_crouch = action_state.pressed(&Actions::PlayerCrouch);

    for (
        mut player_input_state
    ) in query_player.iter_mut() {
        // Update movement
        player_input_state.pointing = action_point;
        player_input_state.movement_normalized = action_move;

        // Update jumping, sprinting, crouching state
        player_input_state.jumping = action_jump;
        player_input_state.sprinting = action_sprint;
        player_input_state.crouching = action_crouch;

        // println!("movement_nor: {}", player_input_state.movement_normalized.to_string());
        // println!("jumping: {}", player_input_state.jumping);
    }
}