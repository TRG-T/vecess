use crate::pos::Pos;
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
type Fields = [[Piece; 8]; 8];
pub struct Board{
    pub fields: Fields,
    pub possible_moves: Option<[[bool; 8]; 8]>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            fields: Self::generate_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"),
            possible_moves: None,
        }
    }

    fn generate_board(pieces: &str) -> Fields {
        let mut fields: Fields = [[Piece::new(Color::Black, Type::Blank); 8]; 8];
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
                'r' => fields[row][col] = Piece::new(Color::Black, Type::Rook),
                'n' => fields[row][col] = Piece::new(Color::Black, Type::Knight),
                'b' => fields[row][col] = Piece::new(Color::Black, Type::Bishop),
                'q' => fields[row][col] = Piece::new(Color::Black, Type::King),
                'k' => fields[row][col] = Piece::new(Color::Black, Type::Queen),
                'p' => fields[row][col] = Piece::new(Color::Black, Type::Pawn),

                // white pieces
                'R' => fields[row][col] = Piece::new(Color::White, Type::Rook),
                'N' => fields[row][col] = Piece::new(Color::White, Type::Knight),
                'B' => fields[row][col] = Piece::new(Color::White, Type::Bishop),
                'Q' => fields[row][col] = Piece::new(Color::White,  Type::King),
                'K' => fields[row][col] = Piece::new(Color::White, Type::Queen),
                'P' => fields[row][col] = Piece::new(Color::White, Type::Pawn),
                '8' => {
                    for a in 0..8 {
                        fields[row][a] = Piece::new(Color::White, Type::Blank);
                    }
                }
                _ => {}
            }
            col += 1;
        }
        fields
    }

    pub fn get_field(&self, pos: &Pos) -> Piece {
        self.fields[pos.y][pos.x]
    }

    pub fn get_mut_field(&mut self, pos: &Pos) -> &mut Piece {
        &mut self.fields[pos.y][pos.x]
    }

    pub fn print_board(&self, cursor: &Cursor) {
        self.print_board_letters();
        for row in 0..BOARD_SIZE {
            print!("{} ", 8 - row); // print board numbers
            for col in 0..BOARD_SIZE {
                self.print_board_fields(Pos::new(row, col), cursor);
            }
            print!("          info");
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

    fn print_board_fields(&self, pos: Pos, cursor: &Cursor) {
        let color = COLORS[(pos.x + pos.y) % 2];
        let field = self.get_field(&pos);
        if cursor.pos == pos {
            if cursor.move_mode {
                print!("{}", cursor.moving_piece.unwrap().piece_type.as_str().on(COLORS[3]));
            } else {
                print!("{}", field.piece_type.as_str().with(field.color).on(COLORS[2]));
            }
            return;
        }
        if let Some(arr) = self.possible_moves {
            if arr[pos.y][pos.x] {
                print!("{}", field.piece_type.as_str().with(field.color).on(Color::Grey));
                return;
            }
        }
        print!("{}", field.piece_type.as_str().with(field.color).on(color));
    }
}