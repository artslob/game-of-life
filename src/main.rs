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

struct Menu {
    cell_shape_index: usize,
    cell_update_frequency: f32,
}

impl Menu {
    fn new() -> Self {
        Self {
            cell_shape_index: 0,
            cell_update_frequency: 0.5,
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

            if is_play_clicked || is_key_released(KeyCode::Space) {
                // TODO make code fail at compile time
                let cell_shape = match self.cell_shape_index {
                    0 => CellShape::Square,
                    1 => CellShape::Circle,
                    _ => panic!("index out of cell shape array"),
                };
                let gameplay_params = GameplayParams {
                    cell_shape,
                    cell_update_frequency: self.cell_update_frequency as f64,
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
}

struct Gameplay {
    cells: Vec<Vec<Cell>>,
    time: f64,
    cell_shape: CellShape,
    cell_update_frequency: f64,
}

impl Gameplay {
    fn new(params: GameplayParams) -> Self {
        Self {
            cells: Self::create_initial_cells(),
            time: get_time(),
            cell_shape: params.cell_shape,
            cell_update_frequency: params.cell_update_frequency,
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
            self.cells = calculate_next_generation(&self.cells);
            self.time = get_time();
        }

        GameState::Playing(self)
    }
}
