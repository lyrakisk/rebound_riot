use bevy::prelude::*;
pub mod ball;
pub mod paddle;
pub mod position;

use ball::*;
use paddle::*;

pub const SPAWN_COMPONENTS: (
    fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>),
    fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>),
    fn(Commands),
) = (spawn_ball, spawn_paddle, spawn_camera);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
