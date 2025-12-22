use bevy::prelude::*;

use crate::components::*;

pub fn spawn_entities(mut commands: Commands) {
    commands.spawn(Ball);
    commands.spawn(Camera2d);
}

pub fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}
