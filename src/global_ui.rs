use bevy::{core_pipeline::core_2d::Camera2d, ecs::system::Commands};

pub fn global_ui_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
