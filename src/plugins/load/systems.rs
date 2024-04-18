use bevy::asset::{AssetPath, LoadState};
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::state::app::{AppGameState, AppLoadState, AppScreenState};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum AssetCategory {
    Model,
    Audio,
    Image
}

#[derive(Clone)]
pub(super) struct AssetLoad {
    category: AssetCategory,
    handle: UntypedHandle
}

#[derive(Resource)]
pub(super) struct AssetLoadBank {
    pub loads: Vec<AssetLoad>
}

#[derive(Component)]
pub(super) struct AssetLoadedMarker;

pub(super) fn queue_assets(
    asset_server: Res<AssetServer>,
    mut bank: ResMut<AssetLoadBank>
) {
    let to_queue = HashMap::from([
        (AssetCategory::Model, vec![
            "character.glb"
        ]),
        (AssetCategory::Image, vec![
            "Briefs_color.jpg",
            "cloth_color.jpg",
            "eye_color.jpg",
            "face_color.jpg",
            "face_Roughness.jpg",
            "hair_color.jpg",
            "prototype_texture1.png",
            "shoe_color.jpg",
            "teeth_bump.jpg",
            "teeth_color.jpg"
        ])
    ]);

    for (category, paths) in to_queue {
        for path in paths {
            let path = path.to_string();
            let asset_path = AssetPath::from(path.clone());

            let handle = asset_server.load_untyped(asset_path);

            let load = AssetLoad {
                category,
                handle: handle.untyped()
            };

            bank.loads.push(load);
        }
    }
}

pub(super) fn update_state(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut bank: ResMut<AssetLoadBank>,
    mut screen_next_state: ResMut<NextState<AppScreenState>>,
    mut load_next_state: ResMut<NextState<AppLoadState>>
) {
    let mut not_loaded = 0;
    let mut loading = 0;
    let mut loaded = 0;
    let mut failed = 0;

    let mut currently_loading = None;

    let loads = &bank.loads;

    for load in loads {
        if let Some(state) = server.get_load_state(load.clone().handle) {
            match state {
                LoadState::NotLoaded => { not_loaded += 1; }
                LoadState::Loading => {
                    loading += 1;
                    currently_loading = Some(load.clone().handle);
                }
                LoadState::Loaded => { loaded += 1; }
                LoadState::Failed => { failed += 1; }
            }
        }
    }

    if not_loaded == 0 {
        // Load has been finished

        info!("Finished loading sequence");

        load_next_state.set(AppLoadState::Loaded);
        if failed > 0 {
            // Fatal: Failed to load some assets
            error!("Failed to load {failed} assets out of {loaded} assets");

            screen_next_state.set(AppScreenState::LoadError)
        } else {
            info!("Successfully pre-loaded all assets, updating state");

            screen_next_state.set(AppScreenState::Main)
        }
    }

    if let Some(handle) = currently_loading {
        if let Some(path) = server.get_path(handle) {
            info!(
                "-----------------\n\
                Path: {path}\n\
                Loaded {loaded} | Loading {loading} | Failed {failed} | NotLoaded {not_loaded}\n\
                -----------------\n"
            )
        }
    }

    // println!("Load sequence is end")
}