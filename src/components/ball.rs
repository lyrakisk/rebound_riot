use super::position::*;
use bevy::prelude::*;

#[derive(Component)]
#[require(Position)]
struct Ball;

pub fn spawn_ball(mut commands: Commands) {
    commands.spawn(Ball);
}
