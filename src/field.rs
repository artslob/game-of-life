use crate::gameplay_params::{CellShape, FieldBorders, GameplayParams, MapGeneration};
use itertools::Itertools;
use macroquad::prelude::*;

pub struct Field {
    cells_a: Vec<Cell>,
    cells_b: Vec<Cell>,
    cells_pointer: CellsPointer,
    width: usize,
    field_borders: FieldBorders,
    cell_shape: CellShape,
    cell_color: Color,
}

impl Field {
    pub fn new(params: &GameplayParams) -> Self {
        let width = 50;
        let cells = match params.map_generation {
            MapGeneration::Random => Self::map_random(width),
            MapGeneration::Glider => Self::map_glider(width),
        };
        Field {
            cells_a: cells.clone(),
            cells_b: cells,
            cells_pointer: CellsPointer::First,
            width,
            field_borders: params.field_borders,
            cell_shape: params.cell_shape,
            cell_color: params.cell_color,
        }
    }

    fn map_glider(width: usize) -> Vec<Cell> {
        let mut cells = (0..(width.pow(2)))
            .map(|_| Cell {
                state: CellState::Dead,
            })
            .collect_vec();

        cells[1].state = CellState::Life;
        cells[width + 2].state = CellState::Life;
        cells[2 * width].state = CellState::Life;
        cells[2 * width + 1].state = CellState::Life;
        cells[2 * width + 2].state = CellState::Life;

        cells
    }

    fn map_random(width: usize) -> Vec<Cell> {
        let seed = (macroquad::time::get_time() * 10_000.0) as u64;
        macroquad::rand::srand(seed);

        (0..(width.pow(2)))
            .map(|_| Cell {
                state: match ::macroquad::rand::gen_range(0u8, 2) {
                    0 => CellState::Life,
                    _ => CellState::Dead,
                },
            })
            .collect()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    pub fn draw(&self, x: f32, y: f32, cell_width: f32) {
        let cells = match self.cells_pointer {
            CellsPointer::First => &self.cells_a,
            CellsPointer::Second => &self.cells_b,
        };
        for (i, row) in cells.chunks(self.width).enumerate() {
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
        for i in 0..self.width {
            for j in 0..self.width {
                let current = match self.cells_pointer {
                    CellsPointer::First => self.cells_a.as_slice(),
                    CellsPointer::Second => self.cells_b.as_slice(),
                };
                let alive_count = count_alive_cells(
                    current,
                    self.width,
                    i as usize,
                    j as usize,
                    self.field_borders,
                );
                let index = self.get_index(i, j);
                let cell = &current[index];
                let state = match (cell.state, alive_count) {
                    (CellState::Dead, 3) => CellState::Life,
                    (CellState::Life, 2 | 3) => CellState::Life,
                    _ => CellState::Dead,
                };
                let next = match self.cells_pointer {
                    CellsPointer::First => &mut self.cells_b,
                    CellsPointer::Second => &mut self.cells_a,
                };
                next[index] = Cell { state };
            }
        }
        self.cells_pointer = self.cells_pointer.swap();
    }
}

fn count_alive_cells(
    current: &[Cell],
    width: usize,
    i: usize,
    j: usize,
    field_borders: FieldBorders,
) -> usize {
    let i_iter = field_borders.create_index_iter(i, width);
    let j_iter = field_borders.create_index_iter(j, width);

    i_iter
        .cartesian_product(j_iter)
        .filter(|(a, b)| !(*a == i && *b == j))
        .map(|(a, b)| &current[(a * width) + b])
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
