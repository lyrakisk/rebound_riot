use super::events::Scored;
use crate::components::ball::{BALL_SPEED, Ball};
use crate::components::position::Position;
use crate::components::velocity::Velocity;
use bevy::prelude::*;

pub fn reset_ball(_event: On<Scored>, ball: Single<(&mut Position, &mut Velocity), With<Ball>>) {
    let (mut ball_position, mut ball_velocity) = ball.into_inner();
    ball_position.0 = Vec2::ZERO;
    ball_velocity.0 = Vec2::new(BALL_SPEED, BALL_SPEED);
}
