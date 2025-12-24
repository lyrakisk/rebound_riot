use super::position::*;
use super::velocity::*;
use bevy::prelude::*;

const BALL_SIZE: f32 = 10.0;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOR: Color = Color::srgb(1., 0., 0.);
const BALL_SPEED: f32 = 2.;

#[derive(Component)]
#[require(Position, Velocity)]
pub struct Ball;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);
    commands.spawn((
        Ball,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Velocity(Vec2::new(BALL_SPEED, BALL_SPEED)),
    ));
}

pub fn move_ball(ball: Single<(&mut Position, &Velocity), With<Ball>>) {
    let (mut position, velocity) = ball.into_inner();
    position.0 += velocity.0 * BALL_SPEED;
}
