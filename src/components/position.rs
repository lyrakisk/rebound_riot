use bevy::prelude::*;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(pub Vec2);
