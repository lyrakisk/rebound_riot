use bevy::prelude::*;

use crate::components::{ball::*, position::*};

pub fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

pub fn move_ball(mut ball: Query<&mut Position, With<Ball>>) {
    if let Ok(mut position) = ball.single_mut() {
        position.0.x += 1.0
    }
}
