use std::{io, time::Duration};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode},
    execute, terminal,
};

use tui_canvas::Grid;
use tui_snake::{Direction, Snake, Apple, render_snake, out_of_bounds};


fn main() {
    if let Ok(score) = play_game() {
        println!("You scored {} points!", score);
    }
}

fn play_game() -> io::Result<usize> {
    let mut stdout = io::stdout();

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide,)?;

    terminal::enable_raw_mode()?;

    let mut grid = Grid::new(30, 30);

    let mut dir = Direction::Down;
    let mut snake = Snake::default().add_segment((0, 1)).add_segment((0, 2));

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

        if out_of_bounds(&new_snake, &snake, &grid)  || new_snake.colliding_with_self() {
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
