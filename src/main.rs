pub mod field;
pub mod game_state;
pub mod gameplay;
pub mod gameplay_params;
pub mod menu;

use crate::game_state::GameState;
use crate::menu::Menu;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game_state = GameState::Menu(Menu::new());

    loop {
        clear_background(game_state.background_color());

        game_state = match game_state {
            GameState::Menu(menu) => menu.show(),
            GameState::Playing(gameplay) => gameplay.play(),
        };

        next_frame().await
    }
}
