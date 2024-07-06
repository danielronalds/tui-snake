use crossterm::style::Color;
use rand::{rngs::ThreadRng, Rng};
use tui_canvas::{Cell, Grid};
use tui_snake::Snake;

pub fn place_apple(grid: &mut Grid, snake: &Snake) -> (u8, u8) {
    let mut rng = rand::thread_rng();

    let mut apple = gen_apple(&mut rng, &grid);

    while snake.occupies(&apple) {
        apple = gen_apple(&mut rng, &grid);
    }

    let _ = grid.set_cell(
        apple.0 as usize,
        apple.1 as usize,
        Cell::build(Color::Red, "  "),
    );

    apple
}

pub fn gen_apple(rng: &mut ThreadRng, grid: &Grid) -> (u8, u8) {
    (
        rng.gen_range(1..(grid.width() - 2) as u8),
        rng.gen_range(1..(grid.height() - 2) as u8),
    )
}
