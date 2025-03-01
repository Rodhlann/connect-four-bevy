use bevy::{color::palettes::css::*, prelude::*};

use crate::{
    logic::{get_lowest_tile_position, is_connect_four, is_tie_game},
    types::{GameState, Position},
};

const PLAYER_ONE_BUTTON: Srgba = Srgba::RED;
const PLAYER_TWO_BUTTON: Srgba = Srgba::rgb(225.0, 225.0, 0.0);
const HINT_BUTTON: Srgba = Srgba::new(0.17, 0.17, 0.17, 1.0);

pub fn grid_interaction_system(
    mut interaction_query: Query<(&Interaction, &Position), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<GameState>,
) {
    if state.game_over {
        return;
    }

    for (interaction, position) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let cell = state.grid[position.1][position.0];
                if cell == 0 {
                    let updated_position =
                        get_lowest_tile_position(position.0, position.1, state.grid);
                    if state.player == 0 {
                        state.player = 1;
                        state.grid[updated_position[1]][updated_position[0]] = 1
                    } else {
                        state.player = 0;
                        state.grid[updated_position[1]][updated_position[0]] = 2
                    }

                    if is_connect_four(state.grid) {
                        state.game_over = true;
                    }

                    if is_tie_game(state.grid) {
                        state.game_over = true;
                        state.tie_game = true;
                    }
                }
            }
            Interaction::Hovered => {
                state.hovered_col = Some(position.0);
            }
            _ => (),
        }
    }
}

pub fn grid_visuals_system(
    mut button_query: Query<(&mut BackgroundColor, &Position), With<Button>>,
    state: ResMut<GameState>,
) {
    for (mut color, position) in &mut button_query {
        let cell = state.grid[position.1][position.0];
        match cell {
            1 => *color = PLAYER_ONE_BUTTON.into(),
            2 => *color = PLAYER_TWO_BUTTON.into(),
            _ => {
                if state.hovered_col == Some(position.0) {
                    *color = HINT_BUTTON.into();
                } else {
                    *color = BLACK.into();
                }
            }
        }
    }
}

pub fn grid_ui_setup(mut commands: Commands) {
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
