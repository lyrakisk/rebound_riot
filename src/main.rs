mod components;
mod systems;

use bevy::prelude::*;
use components::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, SPAWN_COMPONENTS)
        .add_systems(FixedUpdate, project_positions)
        .run();
}
