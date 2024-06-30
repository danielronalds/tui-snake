use std::io;

use crossterm::{cursor, event, execute, terminal};

use tui_canvas::{Cell, Grid};

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut stdout = io::stdout();

    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        event::EnableMouseCapture
    )?;

    terminal::enable_raw_mode()?;

    let grid = Grid::new_full_screen()?;

    Ok(())
}
