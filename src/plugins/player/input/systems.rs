use bevy::prelude::{Query, Vec2, Vec3, With};
use leafwing_input_manager::action_state::ActionState;
use crate::plugins::input::actions::Actions;
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

    let movement = action_state.axis_pair(&Actions::PlayerMove).unwrap().xy();

    for (input_state) in query_player.iter_mut() {
        input_state.movement = Vec3 {
            x: movement.x,
            y: 0.0,
            z: movement.y
        };

        input_state.sprint = action_state.pressed(&Actions::PlayerSprint);
        input_state.crouch = action_state.pressed(&Actions::PlayerCrouch);

        input_state.jump = action_state.pressed(&Actions::PlayerJump);
    }
}