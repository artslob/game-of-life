mod cli;

use crate::cli::CellShape;
use itertools::Itertools;
use macroquad::hash;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

const CELL_COUNT: i32 = 50;

#[macroquad::main("Game of Life")]
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

fn calculate_next_generation(current: &[Vec<Cell>], field_borders: FieldBorders) -> Vec<Vec<Cell>> {
    let mut result = vec![];
    for (i, row) in current.iter().enumerate() {
        let mut next_row = vec![];
        for (j, cell) in row.iter().enumerate() {
            let alive_count = count_alive_cells(current, &row, i, j, field_borders);
            let state = match (cell.state, alive_count) {
                (CellState::Dead, 3) => CellState::Life,
                (CellState::Life, 2 | 3) => CellState::Life,
                _ => CellState::Dead,
            };
            next_row.push(Cell { state });
        }
        result.push(next_row);
    }
    result
}

fn count_alive_cells(
    current: &[Vec<Cell>],
    row: &[Cell],
    i: usize,
    j: usize,
    field_borders: FieldBorders,
) -> usize {
    let i_upper_pos = match i.checked_sub(1) {
        None => match field_borders {
            FieldBorders::Connected => Some(current.len() - 1),
            FieldBorders::Limited => None,
        },
        Some(i) => Some(i),
    };
    let i_lower_pos: Option<usize> = if i + 1 >= current.len() {
        match field_borders {
            FieldBorders::Connected => Some(0),
            FieldBorders::Limited => None,
        }
    } else {
        Some(i + 1)
    };

    let j_left_pos = match j.checked_sub(1) {
        None => match field_borders {
            FieldBorders::Connected => Some(row.len() - 1),
            FieldBorders::Limited => None,
        },
        Some(i) => Some(i),
    };
    let j_right_pos = if j + 1 >= row.len() {
        match field_borders {
            FieldBorders::Connected => Some(0),
            FieldBorders::Limited => None,
        }
    } else {
        Some(j + 1)
    };

    use std::iter::once;

    (i_upper_pos.iter().chain(once(&i)).chain(i_lower_pos.iter()))
        .copied()
        .cartesian_product((j_left_pos.iter().chain(once(&j)).chain(j_right_pos.iter())).copied())
        .filter(|(a, b)| !(*a == i && *b == j))
        .map(|(a, b)| &current[a][b])
        .filter(|cell| matches!(cell.state, CellState::Life))
        .count()
}

#[derive(Debug, Clone)]
struct Cell {
    state: CellState,
}

#[derive(Debug, Copy, Clone)]
enum CellState {
    Dead,
    Life,
}

enum GameState {
    Menu(Menu),
    Playing(Gameplay),
}

impl GameState {
    fn background_color(&self) -> Color {
        match self {
            GameState::Menu(_) => LIGHTGRAY,
            GameState::Playing(_) => BLACK,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum FieldBorders {
    Connected,
    Limited,
}

struct Menu {
    cell_shape_index: usize,
    cell_update_frequency: f32,
    grid_line_thickness: f32,
    field_borders_index: usize,
}

impl Menu {
    fn new() -> Self {
        Self {
            cell_shape_index: 0,
            cell_update_frequency: 0.5,
            grid_line_thickness: 1.5,
            field_borders_index: 0,
        }
    }

    fn show(mut self) -> GameState {
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

struct GameplayParams {
    cell_shape: CellShape,
    cell_update_frequency: f64,
    grid_line_thickness: f32,
    field_borders: FieldBorders,
}

struct Gameplay {
    cells: Vec<Vec<Cell>>,
    time: f64,
    cell_shape: CellShape,
    cell_update_frequency: f64,
    grid_line_thickness: f32,
    field_borders: FieldBorders,
}

impl Gameplay {
    fn new(params: GameplayParams) -> Self {
        Self {
            cells: Self::create_initial_cells(),
            time: get_time(),
            cell_shape: params.cell_shape,
            cell_update_frequency: params.cell_update_frequency,
            grid_line_thickness: params.grid_line_thickness,
            field_borders: params.field_borders,
        }
    }

    fn create_initial_cells() -> Vec<Vec<Cell>> {
        // let life_dead = [CellState::Life, CellState::Dead];
        // let cell_row = (0..CELL_COUNT)
        //     .zip(life_dead.iter().cycle())
        //     .map(|(_, state)| Cell { state: *state })
        //     .collect::<Vec<_>>();
        //
        // let mut cells = (0..CELL_COUNT)
        //     .map(|_| cell_row.clone())
        //     .collect::<Vec<_>>();

        let mut cells: Vec<Vec<Cell>> = (0..CELL_COUNT)
            .map(|_| {
                (0..CELL_COUNT)
                    .map(|_| Cell {
                        state: CellState::Dead,
                    })
                    .collect()
            })
            .collect();

        cells[0][1] = Cell {
            state: CellState::Life,
        };
        cells[1][2] = Cell {
            state: CellState::Life,
        };
        cells[2][0] = Cell {
            state: CellState::Life,
        };
        cells[2][1] = Cell {
            state: CellState::Life,
        };
        cells[2][2] = Cell {
            state: CellState::Life,
        };

        cells
    }

    fn play(mut self) -> GameState {
        let scr_w: f32 = screen_width();
        let scr_h: f32 = screen_height();

        let square_width = scr_w.min(scr_h);

        let x = ((scr_w - square_width) / 2.).max(0.);
        let y = ((scr_h - square_width) / 2.).max(0.);

        let cell_width = square_width / CELL_COUNT as f32;

        const LINE_COLOR: Color = GRAY;
        for i in 0..=CELL_COUNT {
            let horizontal_y = y + i as f32 * cell_width;
            draw_line(
                x,
                horizontal_y,
                x + square_width,
                horizontal_y,
                self.grid_line_thickness,
                LINE_COLOR,
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
                LINE_COLOR,
            );
        }

        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if matches!(cell.state, CellState::Dead) {
                    continue;
                }
                let cell_x = x + (j as f32 * cell_width);
                let cell_y = y + (i as f32 * cell_width);

                match self.cell_shape {
                    CellShape::Circle => {
                        let radius = cell_width / 2.0;
                        draw_circle(cell_x + radius, cell_y + radius, radius, WHITE);
                    }
                    CellShape::Square => {
                        draw_rectangle(cell_x, cell_y, cell_width, cell_width, WHITE)
                    }
                }
            }
        }

        if get_time() - self.time > self.cell_update_frequency {
            self.cells = calculate_next_generation(&self.cells, self.field_borders);
            self.time = get_time();
        }

        if is_key_pressed(KeyCode::Escape) {
            GameState::Menu(Menu::new())
        } else {
            GameState::Playing(self)
        }
    }
}
