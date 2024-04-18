use bevy::prelude::{NextState, ResMut};
use bevy_egui::{egui, EguiContexts};
use crate::state::app::AppScreenState;

pub(super) fn enter() {

}

pub(super) fn exit() {

}

pub(super) fn update(mut contexts: EguiContexts, mut screen_next_state: ResMut<NextState<AppScreenState>>) {
    egui::Window::new("Main").show(contexts.ctx_mut(), |ui| {
        if ui.button("[Enter] Game").clicked() {
            println!("Entering Game");
            screen_next_state.set(AppScreenState::Game);
        }
    });
}