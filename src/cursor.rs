use crossterm::style::Color;

use crate::Pos;
use crate::board::Board;
use crate::piece::{Piece, Type};

pub struct Cursor<'a> {
    pub pos: Pos,
    pub move_mode: bool,
    pub moving_piece: Option<Piece<'a>>,
    pub old_piece_pos: Option<Pos>,
    pub possible_moves: Option<[[bool; 8]; 8]>,
}

impl<'a> Cursor<'a> {
    pub fn new() -> Cursor<'a> {
        Cursor {
            pos: Pos { x: 4, y: 7 },
            move_mode: false,
            moving_piece: None,
            old_piece_pos: None,
            possible_moves: None,
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

    pub fn take_piece(&mut self, board: &mut Board<'a>, white_move: bool) {
        // if !white_move {
        //     return;
        // }
        self.moving_piece = Some(board.get_field(&self.pos));
        self.possible_moves = Some(self.moving_piece.unwrap().get_piece_moves(&self.pos, board));
        self.old_piece_pos = Some(Pos { x: self.pos.x, y: self.pos.y });
        board.get_mut_field(&self.pos).set_char("   ");
        board.get_mut_field(&self.pos).set_type(Type::Blank);
        self.toggle_move_mode();
    }

    pub fn undo_take_piece(&mut self, board: &mut Board<'a>) {
        let old_field = board.get_mut_field(self.old_piece_pos.as_ref().unwrap());
        old_field.set_char(self.moving_piece.unwrap().char.content());
        old_field.set_type(self.moving_piece.unwrap().piece_type);
        self.possible_moves = None;
        self.moving_piece = None;
    }
}