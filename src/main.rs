use macroquad::prelude::*;

const CELL_COUNT: i32 = 5;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BLACK);

        let scr_w: f32 = screen_width();
        let scr_h: f32 = screen_height();

        let square_width = scr_w.min(scr_h);

        let x = ((scr_w - square_width) / 2.).max(0.);
        let y = ((scr_h - square_width) / 2.).max(0.);

        let cell_width = square_width / CELL_COUNT as f32;

        let true_false = [CellState::Life, CellState::Dead];
        let cell_row = (0..CELL_COUNT)
            .zip(true_false.iter().cycle())
            .map(|(_, state)| Cell { state: *state })
            .collect::<Vec<_>>();

        let cells = (0..CELL_COUNT)
            .map(|_| cell_row.clone())
            .collect::<Vec<_>>();

        for (i, row) in cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let skip = if i % 2 == 0 {
                    !matches!(cell.state, CellState::Life)
                } else {
                    matches!(cell.state, CellState::Life)
                };
                if skip {
                    continue;
                }
                let cell_x = x + (j as f32 * cell_width);
                let cell_y = y + (i as f32 * cell_width);

                draw_rectangle(cell_x, cell_y, cell_width, cell_width, WHITE);
            }
        }

        next_frame().await
    }
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
