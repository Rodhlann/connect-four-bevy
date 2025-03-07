use bevy::{color::palettes::css::*, prelude::*};

use crate::{
    logic::{get_lowest_tile_position, is_connect_four, is_tie_game},
    ui::Position,
};

const PLAYER_ONE_BUTTON: Srgba = Srgba::RED;
const RGB: Srgba = Srgba::rgb(225.0, 225.0, 0.0);
const PLAYER_TWO_BUTTON: Srgba = RGB;
const HINT_BUTTON: Srgba = Srgba::new(0.17, 0.17, 0.17, 1.0);

#[derive(Resource, Default)]
pub struct GameState {
    pub grid: [[u8; 7]; 6],
    pub player: u8,
    pub game_over: bool,
    pub tie_game: bool,
    pub hovered_col: Option<usize>,
}

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>();
        app.add_systems(Update, (grid_interaction_system, grid_visuals_system));
    }
}

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
