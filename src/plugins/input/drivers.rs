use bevy::prelude::Query;
use bevy::window::Window;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::prelude::{ActionState, ActionStateDriver};
use crate::plugins::input::actions::Actions;

pub(super) fn update_cursor_position(
    query_window: Query<(&Window, &ActionStateDriver<Actions>)>,
    mut query_action_state: Query<&mut ActionState<Actions>>
) {
    for (window, driver) in query_window.iter() {
        for entity in driver.targets.iter() {
            let mut action_state = query_action_state
                .get_mut(*entity)
                .expect("Entity does not exist");

            if let Some(position) = window.cursor_position() {
                action_state
                    .action_data_mut_or_default(&driver.action)
                    .axis_pair = Some(DualAxisData::from_xy(position));
            }
        }
    }
}