use std::{io, time::Duration};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute,
    style::Color,
    terminal,
};

use tui_canvas::{Cell, Grid};
use tui_snake::{out_of_bounds, render_snake, Apple, Direction, Snake};

fn main() {
    if let Ok(score) = play_game() {
        println!("You scored {} points!", score);
    }
}

fn play_game() -> io::Result<usize> {
    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide,)?;

    terminal::enable_raw_mode()?;

    let mut grid = Grid::new(32, 32);
    add_border(&mut grid);

    let mut dir = Direction::Down;
    let mut snake = Snake::default()
        .add_segment((1, 2))
        .add_segment((1, 3))
        .add_segment((1, 4))
        .add_segment((1, 5));

    let mut apple = Apple::place(&mut grid, &snake);

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

        if out_of_bounds(&new_snake, &grid) || new_snake.colliding_with_self() {
            break;
        }

        if apple.is_eaten(&new_snake) {
            new_snake = snake.add_segment(apple.pos());
            apple = Apple::place(&mut grid, &new_snake);
        }

        render_snake(&new_snake, &snake, &mut grid);

        grid.draw()?;
        snake = new_snake;
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show,)?;

    terminal::disable_raw_mode()?;

    Ok(snake.score())
}

fn add_border(grid: &mut Grid) {
    let border_cell = Cell::build(Color::White, "  ");

    // Top and bottom
    for i in 0..(grid.width() - 1) {
        let _ = grid.set_cell(i, 0, border_cell.clone());
        let _ = grid.set_cell(i, grid.height() - 1, border_cell.clone());
    }

    // Sides
    for i in 0..(grid.height()) {
        let _ = grid.set_cell(0, i, border_cell.clone());
        let _ = grid.set_cell(grid.width() - 1, i, border_cell.clone());
    }
}
