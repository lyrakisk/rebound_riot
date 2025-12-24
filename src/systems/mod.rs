use bevy::prelude::*;

use crate::components::ball::*;
use crate::components::position::*;
use crate::components::velocity::*;

pub fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}
