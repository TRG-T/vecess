use crate::Pos;
use crate::board::Board;
use crate::piece::{Piece, Type};

pub struct Cursor {
    pub pos: Pos,
    pub move_mode: bool,
    pub moving_piece: Option<Piece>,
    pub old_piece_pos: Option<Pos>,
}

impl Cursor{
    pub fn new() -> Cursor {
        Cursor {
            pos: Pos { x: 4, y: 7 },
            move_mode: false,
            moving_piece: None,
            old_piece_pos: None,
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

    pub fn take_piece(&mut self, board: &mut Board, white_move: bool) {
        // if !white_move {
        //     return;
        // }
        self.moving_piece = Some(board.get_field(&self.pos));
        board.possible_moves = Some(self.moving_piece.unwrap().get_piece_moves(&self.pos, board));
        self.old_piece_pos = Some(Pos { x: self.pos.x, y: self.pos.y });
        board.get_mut_field(&self.pos).set_type(Type::Blank);
        self.toggle_move_mode();
    }

    pub fn undo_take_piece(&mut self, board: &mut Board) {
        let old_field = board.get_mut_field(self.old_piece_pos.as_ref().unwrap());
        let moving_piece = self.moving_piece.unwrap();
        old_field.set_type(moving_piece.piece_type);
        board.possible_moves = None;
        self.moving_piece = None;
    }

    pub fn make_move(&mut self, board: &mut Board, mut white_move: bool) {
        if !board.possible_moves.unwrap()[self.pos.y][self.pos.x] {
            self.undo_take_piece(board);
            self.toggle_move_mode();
            return;
        }
        let mut moving_piece = self.moving_piece.unwrap();
        moving_piece.has_moved = true;
        board.fields[self.pos.y][self.pos.x] = moving_piece;
        self.toggle_move_mode();
        board.possible_moves = None;
        white_move = !white_move;
    }
}