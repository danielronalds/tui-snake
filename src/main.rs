mod apple;

use std::{io, time::Duration};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute, terminal,
};

use tui_canvas::{Cell, Grid};
use tui_snake::{diff, Direction, Snake};

use crossterm::style::Color;

use apple::place_apple;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide,)?;

    terminal::enable_raw_mode()?;

    let mut grid = Grid::new(30, 30);

    let mut snake = Snake::default().add_segment((0, 1)).add_segment((0, 2));

    let snake_cell = Cell::build(Color::Green, "  ");

    let mut apple = place_apple(&mut grid, &snake);

    let _ = grid.set_cell(0, 0, snake_cell.clone());

    let mut dir = Direction::Down;

    loop {
        if poll(Duration::from_millis(80))? {
            if let Event::Key(key) = read()? {
                match key.code {
                    KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('s') => {
                        if dir != Direction::Up {
                            dir = Direction::Down
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('w') => {
                        if dir != Direction::Down {
                            dir = Direction::Up
                        }
                    }
                    KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('a') => {
                        if dir != Direction::Right {
                            dir = Direction::Left
                        }
                    }
                    KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('d') => {
                        if dir != Direction::Left {
                            dir = Direction::Right
                        }
                    }
                    KeyCode::Char('q') => break,
                    _ => (),
                }
            }
        }

        let mut new_snake = snake.shift(dir);

        if out_of_bounds(&new_snake, &snake, &grid) {
            break;
        }

        if new_snake.head() == apple {
            new_snake = snake.add_segment(apple);
            apple = place_apple(&mut grid, &new_snake);
        }

        render_snake(&new_snake, &snake, &mut grid);

        grid.draw()?;
        snake = new_snake;
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show,)?;

    terminal::disable_raw_mode()?;

    println!("You scored {} points!", snake.score());

    Ok(())
}


fn out_of_bounds(new_snake: &Snake, old_snake: &Snake, grid: &Grid) -> bool {
    let (new_x, new_y) = new_snake.head();

    new_snake.head() == old_snake.head()
        || new_x as usize == grid.width()
        || new_y as usize == grid.height()
}

fn render_snake(new_snake: &Snake, old_snake: &Snake, grid: &mut Grid) {
    let snake_cell = Cell::build(Color::Green, "  ");

    let (cells_to_delete, cells_to_add) = diff(&old_snake, &new_snake);

    for cell in cells_to_delete {
        let _ = grid.set_cell(cell.0.into(), cell.1.into(), None);
    }

    for cell in cells_to_add {
        let _ = grid.set_cell(cell.0.into(), cell.1.into(), snake_cell.clone());
    }
}
