use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute,
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
};
use std::{
    io::{stdout, Error},
    thread,
    time::Duration,
};

mod board;
use crate::board::Board;
mod piece;
mod cursor;
use crate::cursor::Cursor;

fn fps(fps: u64) -> Duration {
    Duration::from_millis(1000 / fps)
}

fn main() -> Result<(), Error> {
    let mut board = Board::new();
    let white_move = true;
    let mut cursor = Cursor::new();

    enable_raw_mode()?;

    loop {
        clear_terminal()?;
        board.print_board(&cursor);
        match white_move {
            true => println!("       White's move\r"),
            false => println!("      Black's move\r"),
        }
        if poll(Duration::from_millis(500))? {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') => cursor.up(),
                    KeyCode::Char('s') => cursor.down(),
                    KeyCode::Char('a') => cursor.left(),
                    KeyCode::Char('d') => cursor.right(),
                    KeyCode::Enter => {
                        if cursor.move_mode {
                            board.make_move(&mut cursor)
                        } else {
                            cursor.take_piece(&mut board)
                        }
                        cursor.toggle_move_mode()
                    }
                    _ => {}
                }
            }
        }
        thread::sleep(fps(10));
    }
    Ok(())
}

fn clear_terminal() -> Result<(), Error> {
    execute!(
        stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        Hide,
        MoveTo(0, 1),
    )?;
    Ok(())
}
