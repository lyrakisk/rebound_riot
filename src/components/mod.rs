use bevy::prelude::*;
pub mod ball;
pub mod collider;
pub mod gutter;
pub mod paddle;
pub mod players;
pub mod position;
pub mod velocity;

use ball::*;
use gutter::*;
use paddle::*;

pub const SPAWN_COMPONENTS: (
    fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>),
    fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>, Single<&Window>),
    fn(Commands, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>, Single<&Window>),
    fn(Commands),
) = (spawn_ball, spawn_paddles, spawn_gutters, spawn_camera);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
