use bevy::{
    self,
    ecs::{component::Component, system::Resource},
};

#[derive(Resource, Default)]
pub struct GameState {
    pub grid: [[u8; 7]; 6],
    pub player: u8,
    pub game_over: bool,
    pub tie_game: bool,
    pub hovered_col: Option<usize>,
}

#[derive(Component)]
pub struct Position(pub usize, pub usize);

#[derive(Component)]
pub struct GameOverMenu;

#[derive(Component)]
pub struct StartMenu;
