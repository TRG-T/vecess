use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, Stylize, StyledContent},
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
};
use std::{
    io::{stdout, Error},
    thread,
    time::Duration,
};

const BOARD_SIZE: usize = 8;
const LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const COLORS: [Color; 3] = [Color::DarkBlue, Color::Blue, Color::Green];

fn fps(fps: u64) -> Duration {
    Duration::from_millis(1000 / fps)
}

fn main() -> Result<(),Error>{
    let mut board: [[StyledContent<&str>; 8]; 8] = [["   ".black(); 8]; 8];
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
                    KeyCode::Char('s') => cursor.0 += 1,
                    KeyCode::Char('a') => cursor.1 -= 1,
                    KeyCode::Char('d') => cursor.1 += 1,
                    //KeyCode::Enter => show_moves(&mut board, cursor),
                    _ => {}
                },
                Event::Mouse(event) => println!("{:?}", event),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
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

fn print_board(board: [[StyledContent<&str>; 8]; 8], cursor: (usize, usize)) {
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

fn pick_char(row: usize, col: usize, board: [[StyledContent<&str>; 8]; 8], cursor: (usize, usize)) {
    let color = COLORS[(row + col) % 2];
    if cursor == (row, col) {
        print!("{}", board[row][col].on(COLORS[2]));
        return;
    }
    print!("{}", board[row][col].on(color));
}

fn generate_pieces(board: &mut [[StyledContent<&str>; 8]; 8], pieces: &str) {
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
            'r' => board[row][col] = " ♜ ".black(),
            'n' => board[row][col] = " ♞ ".black(),
            'b' => board[row][col] = " ♝ ".black(),
            'q' => board[row][col] = " ♛ ".black(),
            'k' => board[row][col] = " ♚ ".black(),
            'p' => board[row][col] = " ♟ ".black(),
            // white pieces
            'R' => board[row][col] = " ♜ ".white(),
            'N' => board[row][col] = " ♞ ".white(),
            'B' => board[row][col] = " ♝ ".white(),
            'Q' => board[row][col] = " ♛ ".white(),
            'K' => board[row][col] = " ♚ ".white(),
            'P' => board[row][col] = " ♟ ".white(),
            _ => {}
        }
        col += 1;
    }
}