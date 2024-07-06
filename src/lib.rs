mod apple;
pub use apple::Apple;

mod snake;
pub use snake::{diff, Snake, Direction, render_snake, out_of_bounds};
