use crate::gameplay::Gameplay;
use crate::gameplay_params::{CellShape, FieldBorders, GameplayParams, MapGeneration};
use crate::GameState;
use egui_macroquad::egui::{self, Align2, Color32, Rgba, Widget};
use macroquad::prelude::*;

pub struct Menu {
    cell_shape: CellShape,
    cell_update_frequency: f32,
    grid_line_thickness: f32,
    field_borders: FieldBorders,
    map_generation: MapGeneration,
    background_color: Color32,
    cell_color: Color32,
    grid_line_color: Color32,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            cell_shape: CellShape::Square,
            cell_update_frequency: 0.5,
            grid_line_thickness: 1.5,
            field_borders: FieldBorders::Connected,
            map_generation: MapGeneration::Random,
            background_color: Color32::BLACK,
            cell_color: Color32::WHITE,
            grid_line_color: Color32::GRAY,
        }
    }

    pub fn show(mut self) -> GameState {
        let mut gameplay_params = None;

        egui_macroquad::ui(|ctx| {
            egui::Window::new("Game of Life by artslob")
                .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    if screen_height() > screen_width() {
                        ui.set_width(screen_width() * 0.9);
                    } else {
                        // allows to reset width constraints
                        ui.set_max_width(0.0);
                    }

                    let is_play_clicked = ui.horizontal(|ui| ui.button("Play!").clicked()).inner;

                    enum_combobox(ui, "How to generate map", &mut self.map_generation);

                    enum_combobox(ui, "Cell shape", &mut self.cell_shape);

                    enum_combobox(ui, "Field borders", &mut self.field_borders);

                    ui.horizontal(|ui| {
                        ui.label("Choose map update frequency in seconds:");
                        egui::Slider::new(&mut self.cell_update_frequency, 0.01..=10.0).ui(ui);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Choose grid line thickness:");
                        egui::Slider::new(&mut self.grid_line_thickness, 0.0..=1.5).ui(ui);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Choose background color:");
                        ui.color_edit_button_srgba(&mut self.background_color);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Choose cell color:");
                        ui.color_edit_button_srgba(&mut self.cell_color);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Choose grid line color:");
                        ui.color_edit_button_srgba(&mut self.grid_line_color);
                    });

                    ui.separator();

                    ui.vertical_centered(|ui| {
                        ui.hyperlink_to("source code", env!("CARGO_PKG_REPOSITORY"));
                    });

                    if is_play_clicked || is_key_pressed(KeyCode::Enter) {
                        gameplay_params = Some(GameplayParams {
                            cell_update_frequency: self.cell_update_frequency as f64,
                            grid_line_thickness: self.grid_line_thickness,
                            cell_shape: self.cell_shape,
                            field_borders: self.field_borders,
                            map_generation: self.map_generation,
                            background_color: color32_to_color(self.background_color),
                            cell_color: color32_to_color(self.cell_color),
                            grid_line_color: color32_to_color(self.grid_line_color),
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

fn color32_to_color(color32: Color32) -> Color {
    Color::from(Rgba::from(color32).to_array())
}

fn enum_combobox<Value>(ui: &mut egui::Ui, label: &str, value: &mut Value)
where
    Value: PartialEq + strum::IntoEnumIterator,
    for<'a> &'a Value: Into<&'static str>,
{
    let selected_text: &'static str = (value as &Value).into();
    egui::ComboBox::from_label(label)
        .selected_text(selected_text)
        .show_ui(ui, |ui| {
            for variant in Value::iter() {
                let description: &'static str = (&variant).into();
                ui.selectable_value(value, variant, description);
            }
        });
}
