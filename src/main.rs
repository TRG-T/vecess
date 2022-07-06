use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Stylize},
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
};
use std::{
    io::{stdout, Error},
    thread,
    time::Duration,
};

const BOARD_SIZE: usize = 8;
const LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
// 0,154,255 - darker field
// 0,102,204 - brighter field
const COLORS: [Color; 3] = [Color::DarkBlue, Color::Blue, Color::Green];

fn fps(fps: u64) -> Duration {
    Duration::from_millis(1000 / fps)
}

fn main() -> Result<(),Error>{
    let mut board: [[&str; 8]; 8] = [["   "; 8]; 8];
    let white_move = true;
    let mut cursor: (usize, usize) = (7, 4);

    enable_raw_mode()?;
    generate_pieces(&mut board, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    println!();

    loop {
        clear_terminal()?;
        print_board_letters();
        print_board(board, cursor);
        println!();
        match white_move {
            true => println!("       White's move\r"),
            false => println!("      Black's move\r"),
        }
        if poll(Duration::from_millis(500))? {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') => cursor.0 -= 1,
                    _ => {}
                },
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
            }
        }
        thread::sleep(fps(5));
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

fn print_board(board: [[&str; 8]; 8], cursor: (usize, usize)) {
    for row in 0..BOARD_SIZE {
        print!("{} ", 8 - row);
        for col in 0..BOARD_SIZE {
            pick_char(row, col, board, cursor);
        }
        println!("\r");
    }
}

fn print_board_letters() {
    print!("  ");
    for letter in LETTERS {
        print!(" {} ", letter);
    }
    println!("\r");
}

fn pick_char(row: usize, col: usize, board: [[&str; 8]; 8], cursor: (usize, usize)) {
    let color = COLORS[(row + col) % 2];
    if cursor == (row, col) {
        print!("{}", board[row][col].on(COLORS[2]));
        return;
    }
    print!("{}", board[row][col].on(color));
}

fn generate_pieces(board: &mut [[&str; 8]; 8], pieces: &str) {
    let mut row: usize = 0;
    let mut col: usize = 0;
    for c in pieces.chars() {
        if c == '/' {
            row += 1;
            col = 0;
            continue;
        }
        match c {
            // black pieces
            'r' => board[row][col] = " ♖ ",
            'n' => board[row][col] = " ♘ ",
            'b' => board[row][col] = " ♗ ",
            'q' => board[row][col] = " ♕ ",
            'k' => board[row][col] = " ♔ ",
            'p' => board[row][col] = " ♙ ",
            // white pieces
            'R' => board[row][col] = " ♜ ",
            'N' => board[row][col] = " ♞ ",
            'B' => board[row][col] = " ♝ ",
            'Q' => board[row][col] = " ♛ ",
            'K' => board[row][col] = " ♚ ",
            'P' => board[row][col] = " ♟ ",
            _ => {}
        }
        col += 1;
    }
}
