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
const COLORS: [Color; 4] = [Color::DarkBlue, Color::Blue, Color::Green, Color::DarkGrey];
type Board<'a> = [[Piece<'a>;8];8];

fn fps(fps: u64) -> Duration {
    Duration::from_millis(1000 / fps)
}

#[derive(Clone, Copy)]
struct Piece<'a>{
    char: StyledContent<&'a str>,
    has_moved: bool
}

impl<'a> Piece<'a> {
    pub fn new(char: &'a str, fg_color: Color, bg_color: Color) -> Piece<'a> {
        Piece { char: char.with(fg_color).on(bg_color), has_moved: false }
    }
}

fn main() -> Result<(),Error>{
    let mut board: Board = [[Piece::new("  ", Color::Black, Color::Black); 8]; 8];
    let white_move = true;
    let mut cursor: (usize, usize) = (7, 4);

    enable_raw_mode()?;
    generate_board(&mut board, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
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
                    KeyCode::Char('w') => cursor.0 = (cursor.0+7)%8,
                    KeyCode::Char('s') => cursor.0 = (cursor.0+1)%8,
                    KeyCode::Char('a') => cursor.1 = (cursor.1+7)%8,
                    KeyCode::Char('d') => cursor.1 = (cursor.1+1)%8,
                    KeyCode::Enter => {

                    },
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

fn print_board(board: Board, cursor: (usize, usize)) {
    for row in 0..BOARD_SIZE {
        print!("{} ", 8 - row);
        for col in 0..BOARD_SIZE {
            print_board_pieces(row, col, board, cursor);
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

fn print_board_pieces(row: usize, col: usize, board: Board, cursor: (usize, usize)) {
    if cursor == (row, col) {
        print!("{}", board[row][col].char.on(COLORS[2]));
        return;
    }
    print!("{}", board[row][col].char);
}

fn generate_board(board: &mut Board, pieces: &str) {
    let mut row: usize = 0;
    let mut col: usize = 0;
    for c in pieces.chars() {
        if c == '/' {
            row += 1;
            col = 0;
            continue;
        }
        let mut color = COLORS[(row + col) % 2];
        match c {
            // black pieces
            'r' => board[row][col] = Piece::new(" ♜ ", Color::Black, color),
            'n' => board[row][col] = Piece::new(" ♞ ", Color::Black, color),
            'b' => board[row][col] = Piece::new(" ♝ ", Color::Black, color),
            'q' => board[row][col] = Piece::new(" ♛ ", Color::Black, color),
            'k' => board[row][col] = Piece::new(" ♚ ", Color::Black, color),
            'p' => board[row][col] = Piece::new(" ♟ ", Color::Black, color),
            // white pieces
            'R' => board[row][col] = Piece::new(" ♜ ", Color::White, color),
            'N' => board[row][col] = Piece::new(" ♞ ", Color::White, color),
            'B' => board[row][col] = Piece::new(" ♝ ", Color::White, color),
            'Q' => board[row][col] = Piece::new(" ♛ ", Color::White, color),
            'K' => board[row][col] = Piece::new(" ♚ ", Color::White, color),
            'P' => board[row][col] = Piece::new(" ♟ ", Color::White, color),
            '8' => {
                for a in 0..8 {
                    color = COLORS[(row+a)%2];
                    board[row][a] = Piece::new("   ", Color::White, color);
                }
            },
            _ => {}
        }
        col += 1;
    }
}

fn change_background_color(char: &mut StyledContent<&str>, color: Color) {
    char.style_mut().background_color = Some(color);
}