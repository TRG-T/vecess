use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute,
    style::{Color, StyledContent, Stylize},
    terminal::{enable_raw_mode, Clear, ClearType, EnterAlternateScreen},
};
use std::{
    io::{stdout, Error},
    thread,
    time::Duration,
};

const BOARD_SIZE: usize = 8;
const LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const COLORS: [Color; 4] = [
    Color::DarkBlue,
    Color::Blue,
    Color::Green,
    Color::DarkYellow,
];
type Fields<'a> = [[Piece<'a>; 8]; 8];

fn fps(fps: u64) -> Duration {
    Duration::from_millis(1000 / fps)
}

#[derive(Clone, Copy)]
struct Piece<'a> {
    char: StyledContent<&'a str>,
    has_moved: bool,
}

impl<'a> Piece<'a> {
    pub fn new(char: &'a str, fg_color: Color, bg_color: Color) -> Piece<'a> {
        Piece {
            char: char.with(fg_color).on(bg_color),
            has_moved: false,
        }
    }

    pub fn change_background_color(&mut self, color: Color) {
        self.char.style_mut().background_color = Some(color);
    }
}

struct Cursor<'a> {
    x: usize,
    y: usize,
    move_mode: bool,
    moving_piece: Option<Piece<'a>>,
}

impl<'a> Cursor<'a> {
    pub fn new() -> Cursor<'a> {
        Cursor {
            x: 4,
            y: 7,
            move_mode: false,
            moving_piece: None,
        }
    }

    pub fn up(&mut self) {
        self.y = (self.y + 7) % 8;
    }

    pub fn down(&mut self) {
        self.y = (self.y + 1) % 8;
    }

    pub fn right(&mut self) {
        self.x = (self.x + 1) % 8;
    }

    pub fn left(&mut self) {
        self.x = (self.x + 7) % 8;
    }

    pub fn toggle_move_mode(&mut self) {
        self.move_mode = !self.move_mode
    }

    pub fn take_piece(&mut self, board: &mut Board<'a>) {
        self.moving_piece = Some(board.fields[self.y][self.x]);
        // the field from which we take the piece
        let field_style = board.fields[self.y][self.x].char.style();
        board.fields[self.y][self.x] = Piece::new(
            "   ",
            field_style.foreground_color.unwrap(),
            field_style.background_color.unwrap(),
        );
    }
}

struct Board<'a> {
    fields: Fields<'a>,
}

impl<'a> Board<'a> {
    pub fn new() -> Board<'a> {
        Board {
            fields: Self::generate_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
        }
    }

    fn generate_board(pieces: &str) -> Fields<'a> {
        let mut fields: Fields = [[Piece::new("  ", Color::Black, Color::Black); 8]; 8];
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
                'r' => fields[row][col] = Piece::new(" ♜ ", Color::Black, color),
                'n' => fields[row][col] = Piece::new(" ♞ ", Color::Black, color),
                'b' => fields[row][col] = Piece::new(" ♝ ", Color::Black, color),
                'q' => fields[row][col] = Piece::new(" ♛ ", Color::Black, color),
                'k' => fields[row][col] = Piece::new(" ♚ ", Color::Black, color),
                'p' => fields[row][col] = Piece::new(" ♟ ", Color::Black, color),
                // white pieces
                'R' => fields[row][col] = Piece::new(" ♜ ", Color::White, color),
                'N' => fields[row][col] = Piece::new(" ♞ ", Color::White, color),
                'B' => fields[row][col] = Piece::new(" ♝ ", Color::White, color),
                'Q' => fields[row][col] = Piece::new(" ♛ ", Color::White, color),
                'K' => fields[row][col] = Piece::new(" ♚ ", Color::White, color),
                'P' => fields[row][col] = Piece::new(" ♟ ", Color::White, color),
                '8' => {
                    for a in 0..8 {
                        color = COLORS[(row + a) % 2];
                        fields[row][a] = Piece::new("   ", Color::White, color);
                    }
                }
                _ => {}
            }
            col += 1;
        }
        fields
    }

    pub fn print_board(&self, cursor: &Cursor) {
        self.print_board_letters();
        for row in 0..BOARD_SIZE {
            print!("{} ", 8 - row); // print board numbers
            for col in 0..BOARD_SIZE {
                self.print_board_pieces(row, col, cursor);
            }
            println!("\r");
        }
        println!();
    }

    fn print_board_letters(&self) {
        print!("  ");
        for letter in LETTERS {
            print!(" {} ", letter);
        }
        println!("\r");
    }

    fn print_board_pieces(&self, row: usize, col: usize, cursor: &Cursor) {
        if (cursor.y, cursor.x) == (row, col) {
            if cursor.move_mode {
                print!("{}", self.fields[row][col].char.on(COLORS[3]));
            } else {
                print!("{}", self.fields[row][col].char.on(COLORS[2]));
            }
            return;
        }
        print!("{}", self.fields[row][col].char);
    }

    pub fn make_move(&mut self, cursor: &mut Cursor<'a>) {
        let color = self.fields[cursor.y][cursor.x]
            .char
            .style()
            .background_color
            .unwrap();
        let mut moving_piece = cursor.moving_piece.unwrap();
        moving_piece.has_moved = true;
        self.fields[cursor.y][cursor.x] = moving_piece;
        self.fields[cursor.y][cursor.x].change_background_color(color)
    }
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
