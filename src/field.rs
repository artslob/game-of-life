use crate::gameplay_params::{CellShape, FieldBorders, GameplayParams, MapGeneration};
use itertools::Itertools;
use macroquad::prelude::*;

// TODO make private
pub const CELL_COUNT: i32 = 50;

pub struct Field {
    cells_a: Vec<Vec<Cell>>,
    cells_b: Vec<Vec<Cell>>,
    cells_pointer: CellsPointer,
    field_borders: FieldBorders,
    cell_shape: CellShape,
    cell_color: Color,
}

impl Field {
    pub fn new(params: &GameplayParams) -> Self {
        let cells = match params.map_generation {
            MapGeneration::Random => Self::map_random(),
            MapGeneration::Glider => Self::map_glider(),
        };
        Field {
            cells_a: cells.clone(),
            cells_b: cells,
            cells_pointer: CellsPointer::First,
            field_borders: params.field_borders,
            cell_shape: params.cell_shape,
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
        let seed = (macroquad::time::get_time() * 10_000.0) as u64;
        macroquad::rand::srand(seed);

        (0..CELL_COUNT)
            .map(|_| {
                (0..CELL_COUNT)
                    .map(|_| Cell {
                        state: match ::macroquad::rand::gen_range(0u8, 2) {
                            0 => CellState::Life,
                            _ => CellState::Dead,
                        },
                    })
                    .collect()
            })
            .collect()
    }

    pub fn draw(&self, x: f32, y: f32, cell_width: f32) {
        let cells = match self.cells_pointer {
            CellsPointer::First => &self.cells_a,
            CellsPointer::Second => &self.cells_b,
        };
        for (i, row) in cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if matches!(cell.state, CellState::Dead) {
                    continue;
                }
                let cell_x = x + (j as f32 * cell_width);
                let cell_y = y + (i as f32 * cell_width);

                match self.cell_shape {
                    CellShape::Circle => {
                        let radius = cell_width / 2.0;
                        draw_circle(cell_x + radius, cell_y + radius, radius, self.cell_color);
                    }
                    CellShape::Square => {
                        draw_rectangle(cell_x, cell_y, cell_width, cell_width, self.cell_color)
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        let (current, next) = match self.cells_pointer {
            CellsPointer::First => (&self.cells_a, &mut self.cells_b),
            CellsPointer::Second => (&self.cells_b, &mut self.cells_a),
        };
        for (i, row) in current.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let alive_count = count_alive_cells(current, row, i, j, self.field_borders);
                let state = match (cell.state, alive_count) {
                    (CellState::Dead, 3) => CellState::Life,
                    (CellState::Life, 2 | 3) => CellState::Life,
                    _ => CellState::Dead,
                };
                next[i][j] = Cell { state };
            }
        }
        self.cells_pointer = self.cells_pointer.swap();
    }
}

fn count_alive_cells(
    current: &[Vec<Cell>],
    row: &[Cell],
    i: usize,
    j: usize,
    field_borders: FieldBorders,
) -> usize {
    let i_iter = field_borders.create_index_iter(i, current.len());
    let j_iter = field_borders.create_index_iter(j, row.len());

    i_iter
        .cartesian_product(j_iter)
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

#[derive(Debug, Copy, Clone)]
pub enum CellsPointer {
    First,
    Second,
}

impl CellsPointer {
    fn swap(self) -> Self {
        match self {
            CellsPointer::First => CellsPointer::Second,
            CellsPointer::Second => CellsPointer::First,
        }
    }
}
