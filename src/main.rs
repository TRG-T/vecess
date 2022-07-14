use crossterm::terminal::enable_raw_mode;
use game::Game;
use std::io::Error;

mod board;
mod cursor;
mod game;
mod piece;
mod pos;

fn main() -> Result<(), Error> {
    let mut game = Game::new();
    enable_raw_mode()?;
    game.play()?;

    Ok(())
}
