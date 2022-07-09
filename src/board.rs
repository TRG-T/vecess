use crate::piece::Piece;
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