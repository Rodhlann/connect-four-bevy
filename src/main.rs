use bevy::{prelude::*, winit::WinitSettings};

mod gameover;
mod gameplay;
mod logic;
mod ui;

use gameover::GameOverPlugin;
use gameplay::GameplayPlugin;
use ui::UIInitPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Connect Four".into(),
                    resolution: (850.0, 720.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            UIInitPlugin,
            GameplayPlugin,
            GameOverPlugin,
        ))
        .insert_resource(WinitSettings::desktop_app())
        .run();
}
