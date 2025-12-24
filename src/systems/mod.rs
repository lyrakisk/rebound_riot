use bevy::prelude::*;

use crate::components::ball::Ball;
use crate::components::paddle::PADDLE_SPEED;
use crate::components::players::*;
use crate::components::position::Position;
use crate::components::velocity::Velocity;
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

pub fn move_ai(
    ai: Single<(&mut Velocity, &Position), With<Ai>>,
    ball: Single<&Position, With<Ball>>,
) {
    let (mut velocity, position) = ai.into_inner();
    let a_to_b = ball.0 - position.0;
    velocity.0.y = a_to_b.y.signum() * PADDLE_SPEED;
}
