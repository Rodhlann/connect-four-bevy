use crate::{types::GameState, GameOverMenu};
use bevy::{color::palettes::css::*, prelude::*};

pub fn check_game_over(state: Res<GameState>, commands: Commands, query: Query<&GameOverMenu>) {
    if !query.is_empty() {
        return;
    }
    if state.game_over {
        game_over_menu_setup(state, commands);
    }
}

pub fn game_over_interaction_system(
    mut interaction_query: Query<
        (&mut BackgroundColor, &Interaction),
        (Changed<Interaction>, With<Button>, With<GameOverMenu>),
    >,
    mut state: ResMut<GameState>,
    game_over_menu_query: Query<Entity, With<GameOverMenu>>,
    mut commands: Commands,
) {
    for (mut color, interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *state = GameState::default();
                for entity in game_over_menu_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
            }
            Interaction::Hovered => {
                *color = DARK_GREEN.into();
            }
            Interaction::None => *color = GREEN.into(),
        }
    }
}

pub fn game_over_menu_setup(state: Res<GameState>, mut commands: Commands) {
    commands
        .spawn((
            GameOverMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Srgba::new(0.0, 0.0, 0.0, 0.8).into()),
        ))
        .with_children(|parent| {
            let game_over_text = if state.tie_game {
                "Tie Game...".to_string()
            } else {
                format!(
                    "{} Player Won!",
                    if state.player == 0 { "Yellow" } else { "Red" }
                )
            };

            parent.spawn((
                Text::new(game_over_text),
                Node {
                    width: Val::Auto,
                    height: Val::Px(50.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ));
            parent
                .spawn((
                    Button,
                    GameOverMenu,
                    Node {
                        width: Val::Percent(25.0),
                        height: Val::Px(50.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(GREEN.into()),
                ))
                .with_child((Text::new("New Game"),));
        });
}
