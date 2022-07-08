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
const COLORS: [Color; 4] = [Color::DarkBlue, Color::Blue, Color::Green, Color::DarkYellow];
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

struct Cursor<'a> {
    x: usize,
    y: usize,
    move_mode: bool,
    moving_piece: Option<Piece<'a>>
}

impl<'a> Cursor<'a> {
    pub fn new() -> Cursor<'a> {
        Cursor { x: 4, y: 7, move_mode: false, moving_piece: None }
    }

    pub fn toggle_move_mode(&mut self) {
        self.move_mode = !self.move_mode
    }

    pub fn take_piece(&mut self, board: &mut Board<'a>) {
        self.moving_piece = Some(board[self.y][self.x]);
        board[self.y][self.x] = Piece::new("   ", board[self.y][self.x].char.style().foreground_color.unwrap(), board[self.y][self.x].char.style().background_color.unwrap());
    }
}

fn main() -> Result<(),Error>{
    let mut board: Board = [[Piece::new("  ", Color::Black, Color::Black); 8]; 8];
    let white_move = true;
    let mut cursor = Cursor::new();

    enable_raw_mode()?;
    generate_board(&mut board, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    println!();

    loop {
        clear_terminal()?;
        print_board_letters();
        print_board(board, &cursor);
        println!();
        match white_move {
            true => println!("       White's move\r"),
            false => println!("      Black's move\r"),
        }
        if poll(Duration::from_millis(500))? {
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('w') => cursor.y = (cursor.y+7)%8,
                    KeyCode::Char('s') => cursor.y = (cursor.y+1)%8,
                    KeyCode::Char('a') => cursor.x = (cursor.x+7)%8,
                    KeyCode::Char('d') => cursor.x = (cursor.x+1)%8,
                    KeyCode::Enter => {
                        if cursor.move_mode {
                            make_move(&mut cursor, &mut board)
                        } else {
                            cursor.take_piece(&mut board)
                        }
                        cursor.toggle_move_mode()
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

fn print_board(board: Board, cursor: &Cursor) {
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

fn print_board_pieces(row: usize, col: usize, board: Board, cursor: &Cursor) {
    if (cursor.y, cursor.x) == (row, col) {
        if cursor.move_mode {
            print!("{}", board[row][col].char.on(COLORS[3]));
        } else {
            print!("{}", board[row][col].char.on(COLORS[2]));
        }
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

fn make_move<'a: 'b, 'b>(cursor: &mut Cursor<'a>, board: &mut Board<'b>) {
    let color = board[cursor.y][cursor.x].char.style().background_color.unwrap();
    board[cursor.y][cursor.x] = cursor.moving_piece.unwrap();
    change_background_color(&mut board[cursor.y][cursor.x].char, color)
}

fn change_background_color(char: &mut StyledContent<&str>, color: Color) {
    char.style_mut().background_color = Some(color);
}