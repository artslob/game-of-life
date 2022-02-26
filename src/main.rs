mod cli;

use crate::cli::CellShape;
use itertools::Itertools;
use macroquad::prelude::*;

const CELL_COUNT: i32 = 50;

#[macroquad::main("Game of Life")]
async fn main() {
    // let life_dead = [CellState::Life, CellState::Dead];
    // let cell_row = (0..CELL_COUNT)
    //     .zip(life_dead.iter().cycle())
    //     .map(|(_, state)| Cell { state: *state })
    //     .collect::<Vec<_>>();
    //
    // let mut cells = (0..CELL_COUNT)
    //     .map(|_| cell_row.clone())
    //     .collect::<Vec<_>>();
    let cli_args = cli::parse();

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

    let mut time = get_time();

    loop {
        clear_background(BLACK);

        let scr_w: f32 = screen_width();
        let scr_h: f32 = screen_height();

        let square_width = scr_w.min(scr_h);

        let x = ((scr_w - square_width) / 2.).max(0.);
        let y = ((scr_h - square_width) / 2.).max(0.);

        let cell_width = square_width / CELL_COUNT as f32;

        const LINE_THICKNESS: f32 = 1.5;
        const LINE_COLOR: Color = GRAY;
        for i in 0..=CELL_COUNT {
            let horizontal_y = y + i as f32 * cell_width;
            draw_line(
                x,
                horizontal_y,
                x + square_width,
                horizontal_y,
                LINE_THICKNESS,
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
                LINE_THICKNESS,
                LINE_COLOR,
            );
        }

        for (i, row) in cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if matches!(cell.state, CellState::Dead) {
                    continue;
                }
                let cell_x = x + (j as f32 * cell_width);
                let cell_y = y + (i as f32 * cell_width);

                match cli_args.cell_shape {
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

        if get_time() - time > 0.5 {
            cells = calculate_next_generation(&cells);
            time = get_time();
        }

        next_frame().await
    }
}

fn calculate_next_generation(current: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    let mut result = vec![];
    for (i, row) in current.iter().enumerate() {
        let mut next_row = vec![];
        for (j, cell) in row.iter().enumerate() {
            let alive_count = count_alive_cells(current, &row, i, j);
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

fn count_alive_cells(current: &[Vec<Cell>], row: &[Cell], i: usize, j: usize) -> usize {
    let upper_bound = i.checked_sub(1).unwrap_or_default();
    let lower_bound = (i + 1).min(current.len() - 1);
    let left_bound = j.checked_sub(1).unwrap_or_default();
    let right_bound = (j + 1).min(row.len() - 1);
    (upper_bound..=lower_bound)
        .cartesian_product(left_bound..=right_bound)
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
