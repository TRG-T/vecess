use crate::board::Board;
use crate::piece::{Piece, Type};

pub struct Cursor<'a> {
    pub x: usize,
    pub y: usize,
    pub move_mode: bool,
    pub moving_piece: Option<Piece<'a>>,
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
            Type::Blank
        );
    }
}