use bevy::prelude::*;
pub mod ball;
pub mod position;

use ball::*;

pub const SPAWN_COMPONENTS: (fn(Commands), fn(Commands)) = (spawn_ball, spawn_camera);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
