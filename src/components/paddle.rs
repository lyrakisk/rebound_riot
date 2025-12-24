use super::collider::*;
use super::position::*;
use bevy::prelude::*;

const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 100.0;
const PADDLE_SHAPE: Rectangle = Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT);
const PADDLE_COLOR: Color = Color::srgb(0., 1., 0.);

#[derive(Component)]
#[require(Position,   Collider = Collider(PADDLE_SHAPE))]
pub struct Paddle;

pub fn spawn_paddle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(PADDLE_SHAPE);
    let material = materials.add(PADDLE_COLOR);
    commands.spawn((
        Paddle,
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Position(Vec2::new(250., 0.)),
    ));
}
