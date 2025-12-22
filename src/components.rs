use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(pub Vec2);

#[derive(Component)]
#[require(Position)]
pub struct Ball;
