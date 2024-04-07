use bevy::prelude::*;
use bevy::utils::HashMap;

enum AssetCategory {
    Model,
    Audio,
    Image
}

struct AssetLoad {
    category: AssetCategory,
    handle: UntypedHandle
}

#[derive(Resource)]
struct AssetLoadState {
    loads: Vec<AssetLoad>
}

pub(super) fn queue_assets(
    asset_server: Res<AssetServer>,
    load_state: ResMut<AssetLoadState>
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

    for (category, path) in to_queue {
        let handle = asset_server.load(path);

        let load = AssetLoad {
            category,
            handle: handle.untyped()
        };

        load_state.loads.push(load);
    }
}