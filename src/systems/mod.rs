use bevy::prelude::*;

use crate::components::ball::Ball;
use crate::components::collider::Collider;
use crate::components::players::*;
use crate::components::position::Position;
use crate::events::events::Scored;

pub fn project_positions(mut positionables: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut positionables {
        transform.translation = position.0.extend(0.);
    }
}

pub fn detect_goal(
    ball_position: Single<&Position, With<Ball>>,
    player: Single<Entity, (With<Player>, Without<Ai>)>,
    ai: Single<Entity, (With<Ai>, Without<Player>)>,
    window: Single<&Window>,
    mut commands: Commands,
) {
    let ball_position = ball_position.into_inner();
    let half_window_size = window.resolution.size() / 2.;

    if ball_position.0.x > half_window_size.x {
        commands.trigger(Scored { scorer: *player });
    }

    if ball_position.0.x < -half_window_size.x {
        commands.trigger(Scored { scorer: *ai });
    }
}
