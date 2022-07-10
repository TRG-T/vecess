use crate::Pos;
use crate::board::Board;
use crate::piece::{Piece, Type};

pub struct Cursor<'a> {
    pub pos: Pos,
    pub move_mode: bool,
    pub moving_piece: Option<Piece<'a>>,
}

impl<'a> Cursor<'a> {
    pub fn new() -> Cursor<'a> {
        Cursor {
            pos: Pos { x: 4, y: 7 },
            move_mode: false,
            moving_piece: None,
        }
    }

    pub fn up(&mut self) {
        self.pos.y = (self.pos.y + 7) % 8;
    }

    pub fn down(&mut self) {
        self.pos.y = (self.pos.y + 1) % 8;
    }

    pub fn right(&mut self) {
        self.pos.x = (self.pos.x + 1) % 8;
    }

    pub fn left(&mut self) {
        self.pos.x = (self.pos.x + 7) % 8;
    }

    pub fn toggle_move_mode(&mut self) {
        self.move_mode = !self.move_mode
    }

    pub fn take_piece(&mut self, board: &mut Board<'a>) {
        self.moving_piece = Some(board.get_field(&self.pos));
        board.get_mut_field(&self.pos).set_char("   ");
        board.get_mut_field(&self.pos).set_type(Type::Blank);
    }
}