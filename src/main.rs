use crossterm::terminal::enable_raw_mode;
use game::Game;
use std::io::Error;

mod board;
mod cursor;
mod game;
mod piece;

#[derive(PartialEq)]
pub struct Pos {
    x: usize,
    y: usize,
}

fn main() -> Result<(), Error> {
    let mut game = Game::new();
    enable_raw_mode()?;
    game.play()?;

    Ok(())
}
