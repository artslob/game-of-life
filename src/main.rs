use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BLACK);

        let scr_w: f32 = screen_width();
        let scr_h: f32 = screen_height();

        let square_width = scr_w.min(scr_h);

        let x = ((scr_w - square_width) / 2.).max(0.);
        let y = ((scr_h - square_width) / 2.).max(0.);
        draw_rectangle(x, y, square_width, square_width, WHITE);

        next_frame().await
    }
}
