use crate::gameplay::{CellShape, FieldBorders, Gameplay, GameplayParams};
use crate::GameState;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

pub struct Menu {
    cell_shape_index: usize,
    cell_update_frequency: f32,
    grid_line_thickness: f32,
    field_borders_index: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            cell_shape_index: 0,
            cell_update_frequency: 0.5,
            grid_line_thickness: 1.5,
            field_borders_index: 0,
        }
    }

    pub fn show(mut self) -> GameState {
        let mut gameplay = None;

        // TODO fix resize
        let window_position = vec2(screen_width() / 4., screen_height() / 4.);
        let window_size = vec2(screen_width() / 2., screen_height() / 2.);

        root_ui().window(hash!(), window_position, window_size, |ui| {
            ui.label(None, "Game of Life");
            let is_play_clicked = ui.button(None, "Play!");

            ui.separator();

            ui.combo_box(
                hash!(),
                "Choose cell shape",
                &["Square", "Circle"],
                &mut self.cell_shape_index,
            );

            ui.separator();

            ui.label(None, "Choose map update frequency in seconds:");
            ui.slider(
                hash!(),
                "[0.01 .. 10]",
                0.01..10.0,
                &mut self.cell_update_frequency,
            );

            ui.label(None, "Choose grid line thickness:");
            ui.slider(
                hash!(),
                "[0.0 .. 5.0]",
                0.0..5.0,
                &mut self.grid_line_thickness,
            );

            ui.combo_box(
                hash!(),
                "Field borders",
                &["Limited", "Connected"],
                &mut self.field_borders_index,
            );

            if is_play_clicked || is_key_pressed(KeyCode::Space) {
                // TODO make code fail at compile time
                let cell_shape = match self.cell_shape_index {
                    0 => CellShape::Square,
                    1 => CellShape::Circle,
                    _ => panic!("index out of cell shape array"),
                };
                let field_borders = match self.field_borders_index {
                    0 => FieldBorders::Limited,
                    1 => FieldBorders::Connected,
                    _ => panic!("index out of field borders array"),
                };
                let gameplay_params = GameplayParams {
                    cell_shape,
                    cell_update_frequency: self.cell_update_frequency as f64,
                    grid_line_thickness: self.grid_line_thickness,
                    field_borders,
                };
                gameplay = Some(Gameplay::new(gameplay_params));
            }
        });

        match gameplay {
            None => GameState::Menu(self),
            Some(gameplay) => GameState::Playing(gameplay),
        }
    }
}
