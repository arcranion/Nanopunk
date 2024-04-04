use bevy::prelude::{KeyCode, MouseButton, Reflect};
use bevy::utils::default;
use leafwing_input_manager::Actionlike;
use leafwing_input_manager::axislike::{AxisType, VirtualAxis};
use leafwing_input_manager::prelude::{DualAxis, InputKind, InputMap, SingleAxis, UserInput, VirtualDPad};

#[derive(Actionlike, PartialEq, Eq, Copy, Clone, Hash, Debug, Reflect)]
pub enum Actions {
    DevOrbit3d,
    DevActiveOrbit,
    DevZoomControl,

    PlayerLook,
    PlayerMove,
    PlayerJump,
    PlayerCrouch,
    PlayerSprint
}

impl Actions {
    pub fn default_input_map() -> InputMap<Actions> {
        let mut map = InputMap::default();

        map.insert(Actions::DevOrbit3d, DualAxis::mouse_motion());
        map.insert(Actions::DevActiveOrbit, KeyCode::Backquote);
        map.insert(Actions::DevActiveOrbit, MouseButton::Right);
        map.insert(Actions::DevZoomControl, SingleAxis::mouse_wheel_y());

        map.insert(Actions::PlayerMove, VirtualDPad::wasd());
        // map.insert(Actions::PlayerLook); // TODO: Update this to custom driver
        map.insert(Actions::PlayerJump, KeyCode::Space);
        map.insert(Actions::PlayerCrouch, KeyCode::ControlLeft);
        map.insert(Actions::PlayerSprint, KeyCode::ShiftLeft);

        return map;
    }
}