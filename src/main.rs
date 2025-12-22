use bevy::prelude::*;

mod components;
mod systems;

use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_entities)
        .add_systems(FixedUpdate, project_positions)
        .run();
}
