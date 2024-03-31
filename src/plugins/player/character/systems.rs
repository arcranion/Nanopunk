use bevy::prelude::{Commands, Entity, Query, With, Without};
use bevy::scene::SceneBundle;
use bevy::utils::default;
use crate::plugins::player::character::components::{PlayerCharacterBundle};

pub(super) fn create_scene(
    // mut commands: Commands,
    // query_player: Query<(Entity, &PlayerCharacterBundle), (
    //     Without<PlayerCharacterRendered>
    // )>,
) {
    // for (entity, character) in query_player.iter() {
    //     commands.entity(entity)
    //         .insert(PlayerCharacterRendered)
    //         .insert(SceneBundle {
    //             scene: character.model.clone(),
    //             ..default()
    //         });
    // }
}