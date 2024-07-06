use crossterm::style::Color;
use rand::{rngs::ThreadRng, Rng};
use tui_canvas::{Cell, Grid};

use crate::Snake;

/// A struct that represents the apple the snake eats
pub struct Apple((u8, u8));

impl Apple {
    /// Places an apple on the grid, ensuring that it is in bounds and not colliding with the snake
    ///
    /// # Parameters
    ///
    /// - `grid` The grid to place the apple on
    /// - `snake` The snakes current position, after eating the last apple
    ///
    /// # Returns
    ///
    /// A new apple in a random location
    pub fn place(grid: &mut Grid, snake: &Snake) -> Apple {
        let mut rng = rand::thread_rng();

        let mut apple = gen_apple(&mut rng, grid);

        while snake.occupies(&apple) {
            apple = gen_apple(&mut rng, grid);
        }

        let _ = grid.set_cell(
            apple.0 as usize,
            apple.1 as usize,
            Cell::build(Color::Red, "  "),
        );

        Apple(apple)
    }

    /// Figures out if an apple has been eaten by the snake
    ///
    /// # Parameters 
    ///
    /// - `snake` The snake that could've eaten the apple
    ///
    /// # Returns
    ///
    /// True if the apple and the snakes head are colliding
    pub fn is_eaten(&self, snake: &Snake) -> bool {
        snake.head() == self.0
    }

    /// Figures out if an apple has been eaten by the snake
    ///
    /// # Returns
    ///
    /// A tuple of two u8s in the format x, y
    pub fn pos(&self) -> (u8, u8) {
        self.0
    }
}

/// A utility function for generating a random x, y tuple in the bounds of the grid
fn gen_apple(rng: &mut ThreadRng, grid: &Grid) -> (u8, u8) {
    (
        rng.gen_range(1..(grid.width() - 2) as u8),
        rng.gen_range(1..(grid.height() - 2) as u8),
    )
}
