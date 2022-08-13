use crate::field::Field;
use crate::field::CELL_COUNT;
use crate::gameplay_params::GameplayParams;
use crate::{GameState, Menu};
use macroquad::prelude::*;

pub struct Gameplay {
    menu: Menu,
    field: Field,
    time: f64,
    updates_per_sec: f64,
    grid_line_thickness: f32,
    background_color: Color,
    grid_line_color: Color,
    pause_state: PauseState,
}

impl Gameplay {
    pub fn new(menu: Menu, params: GameplayParams) -> Self {
        let field = Field::new(&params);

        Self {
            menu,
            field,
            time: get_time(),
            updates_per_sec: params.updates_per_sec,
            grid_line_thickness: params.grid_line_thickness,
            background_color: params.background_color,
            grid_line_color: params.grid_line_color,
            pause_state: PauseState::Playing,
        }
    }

    pub fn play(mut self) -> GameState {
        let scr_w: f32 = screen_width();
        let scr_h: f32 = screen_height();

        let square_width = scr_w.min(scr_h);

        let x = ((scr_w - square_width) / 2.).max(0.);
        let y = ((scr_h - square_width) / 2.).max(0.);

        let cell_width = square_width / CELL_COUNT as f32;

        for i in 0..=CELL_COUNT {
            let horizontal_y = y + i as f32 * cell_width;
            draw_line(
                x,
                horizontal_y,
                x + square_width,
                horizontal_y,
                self.grid_line_thickness,
                self.grid_line_color,
            );
        }
        for j in 0..=CELL_COUNT {
            let vertical_x = x + j as f32 * cell_width;
            draw_line(
                vertical_x,
                y,
                vertical_x,
                y + square_width,
                self.grid_line_thickness,
                self.grid_line_color,
            );
        }

        self.field.draw(x, y, cell_width);

        if is_key_released(KeyCode::Space) {
            self.swap_pause_state()
        }

        if matches!(self.pause_state, PauseState::Playing) {
            let time_delta = get_time() - self.time;
            let updates = (self.updates_per_sec * time_delta) as i32;
            if updates > 0 {
                for _ in 0..updates {
                    self.field.update();
                }
                self.time += updates as f64 / self.updates_per_sec;
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            GameState::Menu(self.menu)
        } else {
            GameState::Playing(self)
        }
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }

    fn swap_pause_state(&mut self) {
        if let PauseState::Paused { time } = self.pause_state {
            self.time += get_time() - time;
        }
        self.pause_state.swap()
    }
}

#[derive(Debug, Copy, Clone)]
enum PauseState {
    Paused { time: f64 },
    Playing,
}

impl PauseState {
    fn swap(&mut self) {
        *self = match self {
            PauseState::Paused { .. } => PauseState::Playing,
            PauseState::Playing => PauseState::Paused { time: get_time() },
        }
    }
}
