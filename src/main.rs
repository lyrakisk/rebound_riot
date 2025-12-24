mod components;
mod events;
mod systems;

use bevy::prelude::*;
use bevy::window::WindowMode;
use components::ball::*;
use components::paddle::*;
use components::*;
use events::handlers::reset_ball;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rebound Riot".to_string(),
                mode: WindowMode::Fullscreen(
                    MonitorSelection::Primary,
                    VideoModeSelection::Current,
                ),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, SPAWN_COMPONENTS)
        .add_systems(
            FixedUpdate,
            (
                handle_player_input.before(move_paddles),
                move_ball.before(project_positions),
                move_paddles.before(project_positions),
                move_ai.before(project_positions),
                project_positions,
                handle_collisions.after(move_ball),
                constrain_paddle_position.after(move_paddles),
                detect_goal.after(move_ball),
            ),
        )
        .add_observer(reset_ball)
        .run();
}
