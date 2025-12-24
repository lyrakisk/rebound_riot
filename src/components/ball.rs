use super::collider::*;
use super::position::*;
use super::velocity::*;
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

const BALL_SIZE: f32 = 10.0;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOR: Color = Color::srgb(1., 0., 0.);
const BALL_SPEED: f32 = 2.;

#[derive(Component)]
#[require(Position, Velocity,  Collider = Collider(Rectangle::new(BALL_SIZE, BALL_SIZE)))]
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

pub fn handle_collisions(
    ball: Single<(&mut Velocity, &Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider), Without<Ball>>,
) {
    let (mut ball_velocity, ball_position, ball_collider) = ball.into_inner();

    for (other_position, other_collider) in &other_things {
        if let Some(collision) = collide_with_side(
            Aabb2d::new(ball_position.0, ball_collider.0.half_size),
            Aabb2d::new(other_position.0, other_collider.0.half_size),
        ) {
            match collision {
                Collision::Left => {
                    ball_velocity.0.x *= -1.;
                }
                Collision::Right => {
                    ball_velocity.0.x *= -1.;
                }
                Collision::Top => {
                    ball_velocity.0.y *= -1.;
                }
                Collision::Bottom => {
                    ball_velocity.0.y *= -1.;
                }
            }
        }
    }
}
