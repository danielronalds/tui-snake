use std::{io, time::Duration};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute, terminal,
};

use tui_canvas::{Cell, Grid};
use tui_snake::{diff, Direction, Snake};

use crossterm::style::Color;

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide,)?;

    terminal::enable_raw_mode()?;

    let mut grid = Grid::new_full_screen()?;

    let mut snake = Snake::default().add_segment((0, 1)).add_segment((0, 2));

    let snake_cell = Cell::build(Color::Green, "  ");

    let _ = grid.set_cell(0, 0, snake_cell.clone());

    let mut dir = Direction::Down;

    loop {
        if poll(Duration::from_millis(64))? {
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

        let new_snake = snake.shift(dir);

        let (new_x, new_y) = new_snake.head();

        let out_of_bounds = new_snake.head() == snake.head()
            || new_x as usize == grid.width()
            || new_y as usize == grid.height();
        if out_of_bounds {
            break;
        }

        let (old, new) = diff(&snake, &new_snake);

        for cell in old {
            let _ = grid.set_cell(cell.0.into(), cell.1.into(), None);
        }

        for cell in new {
            let _ = grid.set_cell(cell.0.into(), cell.1.into(), snake_cell.clone());
        }

        grid.draw()?;
        snake = new_snake;
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show,)?;

    terminal::disable_raw_mode()?;

    Ok(())
}
