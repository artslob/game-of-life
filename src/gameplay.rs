use crate::gameplay_params::{
    BackgroundColor, CellColor, CellShape, FieldBorders, GameplayParams, MapGeneration,
};
use crate::{GameState, Menu};
use itertools::Itertools;
use macroquad::prelude::*;

const CELL_COUNT: i32 = 50;

pub struct Gameplay {
    cells: Vec<Vec<Cell>>,
    time: f64,
    cell_shape: CellShape,
    cell_update_frequency: f64,
    grid_line_thickness: f32,
    field_borders: FieldBorders,
    background_color: BackgroundColor,
    cell_color: CellColor,
}

impl Gameplay {
    pub fn new(params: GameplayParams) -> Self {
        let cells = match params.map_generation {
            MapGeneration::Random => Self::map_random(),
            MapGeneration::Glider => Self::map_glider(),
        };

        Self {
            cells,
            time: get_time(),
            cell_shape: params.cell_shape,
            cell_update_frequency: params.cell_update_frequency,
            grid_line_thickness: params.grid_line_thickness,
            field_borders: params.field_borders,
            background_color: params.background_color,
            cell_color: params.cell_color,
        }
    }

    fn map_glider() -> Vec<Vec<Cell>> {
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

    fn map_random() -> Vec<Vec<Cell>> {
        (0..CELL_COUNT)
            .map(|_| {
                (0..CELL_COUNT)
                    .map(|_| Cell {
                        state: match ::rand::random::<bool>() {
                            true => CellState::Life,
                            false => CellState::Dead,
                        },
                    })
                    .collect()
            })
            .collect()
    }

    pub fn play(mut self) -> GameState {
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

                let color = self.cell_color.color();

                match self.cell_shape {
                    CellShape::Circle => {
                        let radius = cell_width / 2.0;
                        draw_circle(cell_x + radius, cell_y + radius, radius, color);
                    }
                    CellShape::Square => {
                        draw_rectangle(cell_x, cell_y, cell_width, cell_width, color)
                    }
                }
            }
        }

        // TODO calc cells update by past time
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

    pub fn background_color(&self) -> Color {
        self.background_color.color()
    }
}

fn calculate_next_generation(current: &[Vec<Cell>], field_borders: FieldBorders) -> Vec<Vec<Cell>> {
    let mut result = vec![];
    for (i, row) in current.iter().enumerate() {
        let mut next_row = vec![];
        for (j, cell) in row.iter().enumerate() {
            let alive_count = count_alive_cells(current, row, i, j, field_borders);
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
pub struct Cell {
    state: CellState,
}

#[derive(Debug, Copy, Clone)]
pub enum CellState {
    Dead,
    Life,
}
