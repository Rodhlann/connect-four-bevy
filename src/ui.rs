use bevy::{color::palettes::css::*, prelude::*};

use crate::types::Position;

pub struct UIInitPlugin;
impl Plugin for UIInitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (grid_ui_setup, global_ui_setup));
    }
}

fn global_ui_setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn grid_ui_setup(mut commands: Commands) {
    commands
        .spawn((
            Node {
                display: Display::Grid,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(24.0)),
                grid_template_rows: RepeatedGridTrack::flex(6, 1.0),
                grid_template_columns: RepeatedGridTrack::flex(7, 1.0),
                row_gap: Val::Px(12.0),
                column_gap: Val::Px(12.0),
                ..default()
            },
            BackgroundColor(NAVY.into()),
        ))
        .with_children(|parent| {
            for row in 0..6 {
                for col in 0..7 {
                    parent
                        .spawn((
                            Node {
                                display: Display::Grid,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Start,
                                padding: UiRect::all(Val::Px(10.0)),
                                row_gap: Val::Px(20.0),
                                ..default()
                            },
                            BackgroundColor(NAVY.into()),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Position(col, row),
                                Button,
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BorderRadius::all(Val::Percent(50.0)),
                                BackgroundColor(BLACK.into()),
                            ));
                        });
                }
            }
        });
}
