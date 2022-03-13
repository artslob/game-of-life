use crate::gameplay::Gameplay;
use crate::gameplay_params::{
    BackgroundColor, CellColor, CellShape, FieldBorders, GameplayParams, MapGeneration,
};
use crate::GameState;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use strum::VariantNames;

pub struct Menu {
    cell_shape_index: usize,
    cell_update_frequency: f32,
    grid_line_thickness: f32,
    field_borders_index: usize,
    map_generation_index: usize,
    background_color_index: usize,
    cell_color_index: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            cell_shape_index: 0,
            cell_update_frequency: 0.5,
            grid_line_thickness: 1.5,
            field_borders_index: 0,
            map_generation_index: 0,
            background_color_index: 0,
            cell_color_index: 0,
        }
    }

    pub fn show(mut self) -> GameState {
        let mut gameplay_params = None;

        // macroquad does not support window resizing right now
        let window_position = vec2(screen_width() / 4., screen_height() / 4.);
        let window_size = vec2(screen_width() / 2., screen_height() / 2.);

        root_ui().window(hash!(), window_position, window_size, |ui| {
            ui.label(None, "Game of Life");
            let is_play_clicked = ui.button(None, "Play!");

            ui.separator();

            ui.combo_box(
                hash!(),
                "How to generate map",
                MapGeneration::VARIANTS,
                &mut self.map_generation_index,
            );

            ui.separator();

            ui.combo_box(
                hash!(),
                "Choose cell shape",
                CellShape::VARIANTS,
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

            // ui.label(None, "Choose grid line thickness:");
            // ui.slider(
            //     hash!(),
            //     "[0.0 .. 5.0]",
            //     0.0..5.0,
            //     &mut self.grid_line_thickness,
            // );

            ui.combo_box(
                hash!(),
                "Field borders",
                FieldBorders::VARIANTS,
                &mut self.field_borders_index,
            );

            ui.separator();

            ui.combo_box(
                hash!(),
                "Choose background color",
                BackgroundColor::VARIANTS,
                &mut self.background_color_index,
            );

            ui.separator();

            ui.combo_box(
                hash!(),
                "Choose cell color",
                CellColor::VARIANTS,
                &mut self.cell_color_index,
            );

            if is_play_clicked || is_key_pressed(KeyCode::Enter) {
                let cell_shape =
                    CellShape::from_repr(self.cell_shape_index).expect("cell shape index error");
                let field_borders = FieldBorders::from_repr(self.field_borders_index)
                    .expect("field borders index error");
                let map_generation = MapGeneration::from_repr(self.map_generation_index)
                    .expect("map generation index error");
                let background_color = BackgroundColor::from_repr(self.background_color_index)
                    .expect("background color index error");
                let cell_color =
                    CellColor::from_repr(self.cell_color_index).expect("cell color index error");
                gameplay_params = Some(GameplayParams {
                    cell_update_frequency: self.cell_update_frequency as f64,
                    grid_line_thickness: self.grid_line_thickness,
                    cell_shape,
                    field_borders,
                    map_generation,
                    background_color,
                    cell_color,
                });
            }
        });

        match gameplay_params {
            None => GameState::Menu(self),
            Some(params) => GameState::Playing(Gameplay::new(self, params)),
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}
