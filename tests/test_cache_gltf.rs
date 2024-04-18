use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(States, PartialEq, Eq, Debug, Hash, Clone)]
enum TestState {
    Load,
    Ok,
    Spawn
}

#[derive(Resource)]
struct AssetBank {
    pub scene: Handle<Scene>
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(TestState::Load)
        .insert_resource(AssetBank {
            scene: Handle::default()
        })
        .add_systems(Update, (wait_load, spawn))
        .run();
}

fn wait_load(
    state: Res<State<TestState>>,
    mut next_state: ResMut<NextState<TestState>>,
    asset_server: Res<AssetServer>,
    mut asset_bank: ResMut<AssetBank>
) {
    if *state.get() == TestState::Ok {
        return;
    }

    asset_bank.scene = asset_server.load("character.glb#Scene0");
    if let Some(state) = asset_server.get_load_state(&asset_bank.scene) {
        if state != LoadState::Loaded {
            print!("Not loaded");
            return;
        }
    }

    print!("Loaded");
    next_state.set(TestState::Ok)
}

fn spawn(
    mut commands: Commands,
    state: Res<State<TestState>>,
    mut next_state: ResMut<NextState<TestState>>,
    asset_bank: Res<AssetBank>
) {
    if *state.get() != TestState::Ok {
        return;
    }

    commands.spawn(SceneBundle {
        scene: asset_bank.scene.clone(),
        ..default()
    });
}