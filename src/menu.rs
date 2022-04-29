use crate::gameplay::Gameplay;
use crate::gameplay_params::{
    BackgroundColor, CellColor, CellShape, FieldBorders, GameplayParams, MapGeneration,
};
use crate::GameState;
use egui_macroquad::egui::Align2;
use macroquad::prelude::*;

pub struct Menu {
    cell_shape: CellShape,
    cell_update_frequency: f32,
    grid_line_thickness: f32,
    field_borders: FieldBorders,
    map_generation: MapGeneration,
    background_color_index: usize,
    cell_color_index: usize,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            cell_shape: CellShape::Square,
            cell_update_frequency: 0.5,
            grid_line_thickness: 1.5,
            field_borders: FieldBorders::Connected,
            map_generation: MapGeneration::Random,
            background_color_index: 0,
            cell_color_index: 0,
        }
    }

    pub fn show(mut self) -> GameState {
        let mut gameplay_params = None;

        egui_macroquad::ui(|ctx| {
            egui_macroquad::egui::Window::new("Game of Life by artslob")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    let is_play_clicked = ui.button("Play!").clicked();

                    egui_macroquad::egui::ComboBox::from_label("How to generate map")
                        .selected_text(format!("{:?}", self.map_generation))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.map_generation,
                                MapGeneration::Random,
                                "Random",
                            );
                            ui.selectable_value(
                                &mut self.map_generation,
                                MapGeneration::Glider,
                                "Glider",
                            );
                        });

                    egui_macroquad::egui::ComboBox::from_label("Cell shape")
                        .selected_text(format!("{:?}", self.cell_shape))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.cell_shape, CellShape::Square, "Square");
                            ui.selectable_value(&mut self.cell_shape, CellShape::Circle, "Circle");
                        });

                    // TODO map frequency
                    // TODO grid line thickness

                    egui_macroquad::egui::ComboBox::from_label("Field borders")
                        .selected_text(format!("{:?}", self.field_borders))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.field_borders,
                                FieldBorders::Connected,
                                "Connected",
                            );
                            ui.selectable_value(
                                &mut self.field_borders,
                                FieldBorders::Limited,
                                "Limited",
                            );
                        });

                    // TODO background color
                    // TODO cell color

                    if is_play_clicked || is_key_pressed(KeyCode::Enter) {
                        let background_color =
                            BackgroundColor::from_repr(self.background_color_index)
                                .expect("background color index error");
                        let cell_color = CellColor::from_repr(self.cell_color_index)
                            .expect("cell color index error");
                        gameplay_params = Some(GameplayParams {
                            cell_update_frequency: self.cell_update_frequency as f64,
                            grid_line_thickness: self.grid_line_thickness,
                            cell_shape: self.cell_shape,
                            field_borders: self.field_borders,
                            map_generation: self.map_generation,
                            background_color,
                            cell_color,
                        });
                    }
                });
        });

        egui_macroquad::draw();

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
