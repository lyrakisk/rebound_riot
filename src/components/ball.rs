use super::position::*;
use bevy::prelude::*;

const BALL_SIZE: f32 = 10.0;
const BALL_SHAPE: Circle = Circle::new(BALL_SIZE);
const BALL_COLOR: Color = Color::srgb(1., 0., 0.);

#[derive(Component)]
#[require(Position)]
struct Ball;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(BALL_SHAPE);
    let material = materials.add(BALL_COLOR);
    commands.spawn((Ball, Mesh2d(mesh), MeshMaterial2d(material)));
}
