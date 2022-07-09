use crate::piece::{Piece, Type};
use crate::cursor::Cursor;
use crossterm::style::{Color, Stylize};

const COLORS: [Color; 4] = [
    Color::DarkBlue,
    Color::Blue,
    Color::Green,
    Color::DarkYellow,
];
const LETTERS: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
const BOARD_SIZE: usize = 8;
type Fields<'a> = [[Piece<'a>; 8]; 8];
pub struct Board<'a> {
    pub fields: Fields<'a>,
}

impl<'a> Board<'a> {
    pub fn new() -> Board<'a> {
        Board {
            fields: Self::generate_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
        }
    }

    fn generate_board(pieces: &str) -> Fields<'a> {
        let mut fields: Fields = [[Piece::new("  ", Color::Black, Type::Blank); 8]; 8];
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
                'r' => fields[row][col] = Piece::new(" ♜ ", Color::Black, Type::Rook),
                'n' => fields[row][col] = Piece::new(" ♞ ", Color::Black, Type::Knight),
                'b' => fields[row][col] = Piece::new(" ♝ ", Color::Black, Type::Bishop),
                'q' => fields[row][col] = Piece::new(" ♛ ", Color::Black, Type::King),
                'k' => fields[row][col] = Piece::new(" ♚ ", Color::Black, Type::Queen),
                'p' => fields[row][col] = Piece::new(" ♟ ", Color::Black, Type::Pawn),

                // white pieces
                'R' => fields[row][col] = Piece::new(" ♜ ", Color::White, Type::Rook),
                'N' => fields[row][col] = Piece::new(" ♞ ", Color::White, Type::Knight),
                'B' => fields[row][col] = Piece::new(" ♝ ", Color::White, Type::Bishop),
                'Q' => fields[row][col] = Piece::new(" ♛ ", Color::White,  Type::King),
                'K' => fields[row][col] = Piece::new(" ♚ ", Color::White, Type::Queen),
                'P' => fields[row][col] = Piece::new(" ♟ ", Color::White, Type::Pawn),
                '8' => {
                    for a in 0..8 {
                        fields[row][a] = Piece::new("   ", Color::White, Type::Blank);
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
        let color = COLORS[(row + col) % 2];
        if (cursor.y, cursor.x) == (row, col) {
            if cursor.move_mode {
                print!("{}", self.fields[row][col].char.on(COLORS[3]));
            } else {
                print!("{}", self.fields[row][col].char.on(COLORS[2]));
            }
            return;
        }
        print!("{}", self.fields[row][col].char.on(color));
    }

    pub fn make_move(&mut self, cursor: &mut Cursor<'a>) {
        let mut moving_piece = cursor.moving_piece.unwrap();
        moving_piece.has_moved = true;
        self.fields[cursor.y][cursor.x] = moving_piece;
    }
}