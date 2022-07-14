use crate::{board::Board, cursor::Cursor};
use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute,
    style::Stylize,
    terminal::{Clear, ClearType},
};
use std::{
    io::{stdout, Error},
    thread::sleep,
    time::Duration,
};

pub struct Game {
    board: Board,
    cursor: Cursor,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            cursor: Cursor::new(),
        }
    }

    pub fn play(&mut self) -> Result<(), Error> {
        loop {
            self.clear_terminal()?;
            self.board.print_board(&self.cursor);
            match self.cursor.white_move {
                true => println!(
                    "  {}\r",
                    "      White's move      ".black().on_white().bold()
                ),
                false => println!(
                    "  {}\r",
                    "      Black's move      ".black().on_white().bold()
                ),
            }
            if poll(Duration::from_millis(500))? {
                if let Event::Key(event) = read()? {
                    match event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('w') => self.cursor.up(),
                        KeyCode::Char('s') => self.cursor.down(),
                        KeyCode::Char('a') => self.cursor.left(),
                        KeyCode::Char('d') => self.cursor.right(),
                        KeyCode::Enter => {
                            if self.cursor.move_mode {
                                self.cursor.make_move(&mut self.board);
                            } else {
                                self.cursor.take_piece(&mut self.board)
                            }
                        }
                        KeyCode::Esc => {
                            if self.cursor.move_mode {
                                self.cursor.undo_take_piece(&mut self.board);
                                self.cursor.toggle_move_mode();
                            }
                        }
                        _ => {}
                    }
                }
            }
            sleep(self.fps(10));
        }
        Ok(())
    }

    fn clear_terminal(&self) -> Result<(), Error> {
        execute!(stdout(), Clear(ClearType::All), Hide, MoveTo(0, 1),)?;
        Ok(())
    }

    fn fps(&self, fps: u64) -> Duration {
        Duration::from_millis(1000 / fps)
    }
}
