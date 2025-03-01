use bevy::{prelude::*, winit::WinitSettings};

mod logic;

mod types;
use types::{GameOverMenu, GameState};

mod global_ui;
use global_ui::global_ui_setup;

mod grid_ui;
use grid_ui::{grid_interaction_system, grid_ui_setup, grid_visuals_system};

mod menu_ui;
use menu_ui::{check_game_over, game_over_interaction_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Connect Four".into(),
                resolution: (850.0, 720.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(GameState::default())
        .add_systems(Startup, (grid_ui_setup, global_ui_setup))
        .add_systems(
            Update,
            (
                grid_interaction_system,
                grid_visuals_system,
                check_game_over,
                game_over_interaction_system,
            ),
        )
        .run();
}
