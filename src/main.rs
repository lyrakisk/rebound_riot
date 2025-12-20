use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_ball, spawn_camera))
        .run();
}

fn spawn_ball(mut commands: Commands) {
    println!("Spawning ball...");
    commands.spawn(Ball);
}

fn spawn_camera(mut commands: Commands) {
    println!("Spawning camera...");
    commands.spawn(Camera2d);
}

#[derive(Component, Default)]
#[require(Transform)]
struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
struct Ball;
