use crate::gameplay::Gameplay;
use crate::menu::Menu;
use macroquad::prelude::*;

pub enum GameState {
    Menu(Menu),
    Playing(Gameplay),
}

impl GameState {
    pub fn background_color(&self) -> Color {
        match self {
            GameState::Menu(_) => LIGHTGRAY,
            GameState::Playing(_) => BLACK,
        }
    }
}
